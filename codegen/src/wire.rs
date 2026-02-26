use crate::parser::{DawnApi, Definition, MethodDef, RecordMember, ReturnType};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct DawnWireJson {
    #[serde(default)]
    pub commands: BTreeMap<String, Vec<WireRecordMember>>,
    #[serde(rename = "return commands", default)]
    pub return_commands: BTreeMap<String, Vec<WireRecordMember>>,
    #[serde(rename = "special commands", default)]
    pub special_commands: BTreeMap<String, Vec<WireRecordMember>>,
    #[serde(rename = "special items", default)]
    pub special_items: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WireRecordMember {
    pub name: String,
    #[serde(rename = "type")]
    pub member_type: String,
}

pub struct DawnWireJsonParser;

impl DawnWireJsonParser {
    pub fn parse_file(path: &Path) -> Result<DawnWireJson, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let api: DawnWireJson = serde_json::from_str(&content)?;
        Ok(api)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CommandKind {
    Special,
    Command,
    ReturnCommand,
}

#[derive(Debug, Clone)]
struct ComputedCommand {
    name: String,
    kind: CommandKind,
    members: Vec<WireRecordMember>,
    client_handwritten: bool,
}

pub struct WireGeneratedFiles {
    pub wire_types: String,
    pub wire_client: String,
    pub wire_server: String,
}

pub fn generate_wire_files(api: &DawnApi, wire: &DawnWireJson) -> WireGeneratedFiles {
    let commands = compute_wire_commands(api, wire);
    WireGeneratedFiles {
        wire_types: generate_wire_types(&commands),
        wire_client: generate_wire_client(&commands),
        wire_server: generate_wire_server(&commands),
    }
}

fn generate_wire_types(commands: &[ComputedCommand]) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireDirection {
    ClientToServer,
    ServerToClient,
    Special,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WireValue {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    Bool(bool),
    F32(f32),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WireField {
    pub name: &'static str,
    pub value: WireValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WirePacket {
    pub command: WireCommand,
    pub fields: Vec<WireField>,
}

#[derive(Debug, Clone, Copy)]
pub struct WireCommandMeta {
    pub name: &'static str,
    pub direction: WireDirection,
    pub fields: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WireCommand {
"#,
    );

    for cmd in commands {
        out.push_str(&format!("    {},\n", to_variant(cmd)));
    }
    out.push_str("}\n\n");

    for cmd in commands {
        let const_name = format!("{}_FIELDS", to_variant(cmd).to_shouty_snake_case());
        out.push_str(&format!("pub const {const_name}: &[&str] = &[\n"));
        for m in &cmd.members {
            out.push_str(&format!("    {},\n", string_lit(&m.name)));
        }
        out.push_str("];\n\n");
    }

    out.push_str("pub fn wire_command_meta(command: WireCommand) -> WireCommandMeta {\n");
    out.push_str("    match command {\n");
    for cmd in commands {
        let variant = to_variant(cmd);
        let const_name = format!("{}_FIELDS", variant.to_shouty_snake_case());
        let direction = match cmd.kind {
            CommandKind::Special => "WireDirection::Special",
            CommandKind::Command => "WireDirection::ClientToServer",
            CommandKind::ReturnCommand => "WireDirection::ServerToClient",
        };
        out.push_str(&format!(
            "        WireCommand::{variant} => WireCommandMeta {{ name: {name}, direction: {direction}, fields: {const_name} }},\n",
            name = string_lit(&cmd.name)
        ));
    }
    out.push_str("    }\n}\n\n");

    out.push_str("pub fn wire_command_name(command: WireCommand) -> &'static str {\n");
    out.push_str("    wire_command_meta(command).name\n}\n\n");

    out.push_str("pub fn wire_command_from_name(name: &str) -> Option<WireCommand> {\n");
    out.push_str("    match name {\n");
    for cmd in commands {
        out.push_str(&format!(
            "        {name} => Some(WireCommand::{variant}),\n",
            name = string_lit(&cmd.name),
            variant = to_variant(cmd)
        ));
    }
    out.push_str("        _ => None,\n    }\n}\n\n");

    out.push_str(
        r#"pub fn build_packet(command: WireCommand, values: Vec<WireValue>) -> Result<WirePacket, String> {
    let fields = wire_command_meta(command).fields;
    if values.len() != fields.len() {
        return Err(format!(
            "field length mismatch for {}: got {}, expected {}",
            wire_command_name(command),
            values.len(),
            fields.len()
        ));
    }
    let fields = fields
        .iter()
        .zip(values)
        .map(|(name, value)| WireField { name, value })
        .collect();
    Ok(WirePacket { command, fields })
}
"#,
    );

    crate::emitter::format_rust_source(&out)
}

fn generate_wire_client(commands: &[ComputedCommand]) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code)]
use super::wire_types::{build_packet, wire_command_name, WireCommand, WirePacket, WireValue};

#[derive(Debug)]
pub enum WireClientError {
    Encode(String),
    Transport(String),
    UnexpectedCommand(&'static str),
}

pub trait WireClientTransport {
    fn send(&mut self, packet: WirePacket) -> Result<(), String>;
}

pub trait WireReturnHandler {
"#,
    );

    for cmd in commands
        .iter()
        .filter(|c| c.kind == CommandKind::ReturnCommand)
    {
        let method = format!("on_{}", to_method_name(&cmd.name));
        out.push_str(&format!(
            "    fn {method}(&mut self, _packet: &WirePacket) -> Result<(), WireClientError> {{ Ok(()) }}\n",
        ));
    }

    out.push_str("}\n\n");

    out.push_str(
        r#"pub struct WireClient<T: WireClientTransport> {
    transport: T,
}

impl<T: WireClientTransport> WireClient<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }

    pub fn send_command(
        &mut self,
        command: WireCommand,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(command, values).map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }

"#,
    );

    for cmd in commands.iter().filter(|c| {
        matches!(c.kind, CommandKind::Command | CommandKind::Special) && !c.client_handwritten
    }) {
        let method = to_method_name(&cmd.name);
        let variant = to_variant(cmd);
        out.push_str(&format!(
            r#"    pub fn {method}(&mut self, values: Vec<WireValue>) -> Result<(), WireClientError> {{
        let packet = build_packet(WireCommand::{variant}, values).map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }}

"#
        ));
    }

    out.push_str(
        r#"    pub fn handle_return_packet<H: WireReturnHandler>(
        &mut self,
        packet: WirePacket,
        handler: &mut H,
    ) -> Result<(), WireClientError> {
        match packet.command {
"#,
    );

    for cmd in commands
        .iter()
        .filter(|c| c.kind == CommandKind::ReturnCommand)
    {
        out.push_str(&format!(
            "            WireCommand::{variant} => handler.{method}(&packet),\n",
            variant = to_variant(cmd),
            method = format!("on_{}", to_method_name(&cmd.name))
        ));
    }

    out.push_str(
        r#"            other => Err(WireClientError::UnexpectedCommand(wire_command_name(other))),
        }
    }
}
"#,
    );

    crate::emitter::format_rust_source(&out)
}

fn generate_wire_server(commands: &[ComputedCommand]) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code)]
use super::wire_types::{wire_command_name, WireCommand, WirePacket};

#[derive(Debug)]
pub enum WireServerError {
    Handler(String),
    UnexpectedCommand(&'static str),
}

    pub trait WireServerDoer {
"#,
    );
    for cmd in commands
        .iter()
        .filter(|c| matches!(c.kind, CommandKind::Special | CommandKind::Command))
    {
        let method = format!("do_{}", to_method_name(&cmd.name));
        out.push_str(&format!(
            "    fn {method}(&mut self, _packet: &WirePacket) -> Result<(), WireServerError> {{ Err(WireServerError::UnexpectedCommand({name})) }}\n",
            name = string_lit(&cmd.name)
        ));
    }
    out.push_str("}\n\n");
    out.push_str(
        r#"pub struct GeneratedServerDoer {
    responses: Vec<WirePacket>,
}

impl GeneratedServerDoer {
    pub fn new() -> Self {
        Self {
            responses: Vec::new(),
        }
    }

    pub fn pop_response(&mut self) -> Option<WirePacket> {
        if self.responses.is_empty() {
            None
        } else {
            Some(self.responses.remove(0))
        }
    }
}

impl Default for GeneratedServerDoer {
    fn default() -> Self {
        Self::new()
    }
}

impl WireServerDoer for GeneratedServerDoer {
    fn do_instance_request_adapter(
        &mut self,
        packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        use crate::generated::{
            AdapterInfo, FutureWaitInfo, Instance, InstanceDescriptor, InstanceFeatureName,
            RequestAdapterStatus, Status, WaitStatus,
        };
        use super::wire_types::{build_packet, WireCommand, WireValue};
        use std::sync::mpsc;

        let event_manager = packet
            .fields
            .iter()
            .find(|f| f.name == "event manager handle")
            .and_then(|f| match f.value {
                WireValue::U64(v) => Some(v),
                _ => None,
            })
            .unwrap_or(0);
        let future_id = packet
            .fields
            .iter()
            .find(|f| f.name == "future")
            .and_then(|f| match f.value {
                WireValue::U64(v) => Some(v),
                _ => None,
            })
            .unwrap_or(0);

        let mut instance_desc = InstanceDescriptor::new();
        instance_desc.required_features = Some(vec![InstanceFeatureName::TimedWaitAny]);
        let instance = Instance::new(Some(&instance_desc));

        let (tx, rx) = mpsc::channel::<Result<String, String>>();
        let future = instance.request_adapter(None, move |status, adapter, message| {
            if status != RequestAdapterStatus::Success {
                let _ = tx.send(Err(format!("{status:?}: {message}")));
                return;
            }
            let adapter = match adapter {
                Some(v) => v,
                None => {
                    let _ = tx.send(Err("request adapter returned no adapter".to_string()));
                    return;
                }
            };
            let mut info = AdapterInfo::new();
            let get_info_status = adapter.get_info(&mut info);
            if get_info_status != Status::Success {
                let _ = tx.send(Err(format!("adapter.get_info failed: {get_info_status:?}")));
                return;
            }
            let vendor = info.vendor.as_deref().unwrap_or("Unknown");
            let device = info.device.as_deref().unwrap_or("Unknown");
            let description = info.description.as_deref().unwrap_or("Unknown");
            let _ = tx.send(Ok(format!("{vendor} {device} ({description})")));
        });

        let wait = instance.wait_any(
            Some(&mut [FutureWaitInfo {
                future: Some(future),
                completed: None,
            }]),
            0,
        );
        if wait != WaitStatus::Success {
            return Err(WireServerError::Handler(format!(
                "instance.wait_any failed: {wait:?}",
            )));
        }

        let info_string = rx
            .recv()
            .map_err(|e| WireServerError::Handler(e.to_string()))?
            .map_err(WireServerError::Handler)?;

        let response = build_packet(
            WireCommand::ReturnInstanceRequestAdapterCallback,
            vec![
                WireValue::U64(event_manager),
                WireValue::U64(future_id),
                WireValue::U32(0),
                WireValue::String("ok".to_string()),
                WireValue::String(info_string),
                WireValue::Null,
                WireValue::U32(0),
                WireValue::Bytes(Vec::new()),
            ],
        )
        .map_err(WireServerError::Handler)?;
        self.responses.push(response);
        Ok(())
    }
}

"#,
    );

    out.push_str(
        r#"pub struct WireServer<H: WireServerDoer> {
    handler: H,
}

impl<H: WireServerDoer> WireServer<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    pub fn handler_mut(&mut self) -> &mut H {
        &mut self.handler
    }

    pub fn handle_packet(&mut self, packet: WirePacket) -> Result<(), WireServerError> {
        match packet.command {
"#,
    );

    for cmd in commands
        .iter()
        .filter(|c| matches!(c.kind, CommandKind::Special | CommandKind::Command))
    {
        out.push_str(&format!(
            "            WireCommand::{variant} => self.handler.{method}(&packet),\n",
            variant = to_variant(cmd),
            method = format!("do_{}", to_method_name(&cmd.name))
        ));
    }

    out.push_str(
        r#"            other => Err(WireServerError::UnexpectedCommand(wire_command_name(other))),
        }
    }

    pub fn handle_packets<I>(&mut self, packets: I) -> Result<(), WireServerError>
    where
        I: IntoIterator<Item = WirePacket>,
    {
        for packet in packets {
            self.handle_packet(packet)?;
        }
        Ok(())
    }
}
"#,
    );

    crate::emitter::format_rust_source(&out)
}

fn compute_wire_commands(api: &DawnApi, wire: &DawnWireJson) -> Vec<ComputedCommand> {
    let special_items = normalized_special_items(&wire.special_items);
    let mut client_handwritten: BTreeSet<String> = special_items
        .get("client_handwritten_commands")
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect();
    for side in special_items
        .get("client_side_commands")
        .cloned()
        .unwrap_or_default()
    {
        client_handwritten.insert(side);
    }

    let mut commands = Vec::new();

    for (name, members) in &wire.special_commands {
        commands.push(ComputedCommand {
            name: name.clone(),
            kind: CommandKind::Special,
            members: members.clone(),
            client_handwritten: client_handwritten.contains(&to_wire_camel_name(name)),
        });
    }

    for (object_name, def) in api.objects() {
        for method in &def.methods {
            if !is_wire_auto_method(api, method.returns()) {
                continue;
            }
            let name = format!("{object_name} {}", method.name);
            let suffix = to_wire_camel_name(&name);
            if client_handwritten.contains(&suffix) {
                continue;
            }
            commands.push(ComputedCommand {
                name,
                kind: CommandKind::Command,
                members: command_members_from_object_method(api, object_name, method),
                client_handwritten: false,
            });
        }
    }

    for (name, members) in &wire.commands {
        commands.push(ComputedCommand {
            name: name.clone(),
            kind: CommandKind::Command,
            members: members.clone(),
            client_handwritten: client_handwritten.contains(&to_wire_camel_name(name)),
        });
    }

    for (name, members) in &wire.return_commands {
        commands.push(ComputedCommand {
            name: name.clone(),
            kind: CommandKind::ReturnCommand,
            members: members.clone(),
            client_handwritten: false,
        });
    }

    commands.sort_by(|a, b| {
        if a.kind == b.kind {
            a.name.cmp(&b.name)
        } else {
            a.kind.cmp(&b.kind)
        }
    });
    commands
}

fn command_members_from_object_method(
    api: &DawnApi,
    object_name: &str,
    method: &MethodDef,
) -> Vec<WireRecordMember> {
    let mut members = Vec::new();
    members.push(WireRecordMember {
        name: "self".to_string(),
        member_type: object_name.to_string(),
    });
    members.extend(method.args.iter().map(to_wire_member));

    if let Some(ret) = method.returns() {
        if is_object_return(api, ret) {
            members.push(WireRecordMember {
                name: "result".to_string(),
                member_type: "ObjectHandle".to_string(),
            });
        }
    }
    members
}

fn to_wire_member(member: &RecordMember) -> WireRecordMember {
    WireRecordMember {
        name: member.name.clone(),
        member_type: member.member_type.clone(),
    }
}

fn is_wire_auto_method(api: &DawnApi, returns: Option<&ReturnType>) -> bool {
    if returns.is_none() {
        return true;
    }
    let ret = returns.expect("checked is_some");
    is_object_return(api, ret) || is_status_return(ret)
}

fn is_status_return(returns: &ReturnType) -> bool {
    returns.get_type().eq_ignore_ascii_case("status")
}

fn is_object_return(api: &DawnApi, returns: &ReturnType) -> bool {
    matches!(
        api.definitions.get(returns.get_type()),
        Some(Definition::Object(_))
    )
}

fn normalized_special_items(raw: &BTreeMap<String, Value>) -> BTreeMap<String, Vec<String>> {
    let mut out = BTreeMap::new();
    for (name, value) in raw {
        if let Value::Array(items) = value {
            out.insert(
                name.clone(),
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect(),
            );
        }
    }
    out
}

fn to_variant(command: &ComputedCommand) -> String {
    let base = to_wire_camel_name(&command.name);
    match command.kind {
        CommandKind::ReturnCommand => format!("Return{base}"),
        CommandKind::Special | CommandKind::Command => base,
    }
}

fn to_method_name(name: &str) -> String {
    to_wire_camel_name(name).to_snake_case()
}

fn to_wire_camel_name(name: &str) -> String {
    let mut out = String::new();
    for part in name.replace('/', " ").split_whitespace() {
        out.push_str(&part.to_upper_camel_case());
    }
    out.replace("3D", "3d")
        .replace("2D", "2d")
        .replace("1D", "1d")
        .replace("Id", "ID")
        .replace("Api", "API")
        .replace("Wgsl", "WGSL")
}

fn string_lit(value: &str) -> String {
    serde_json::to_string(value).expect("serialize string literal")
}
