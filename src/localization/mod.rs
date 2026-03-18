//
// © 2025-2026 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::error::{ErrorRecord, Payload};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "locales/"]
struct Locales;

pub fn format_error(record: &ErrorRecord, locale: &str) -> String {
    // Create a file name by combining the locale and .toml extension
    let filename = format!("{}.toml", locale);
    // Get the file, falling back to English if not found
    let file = Locales::get(&filename).unwrap_or_else(|| Locales::get("en.toml").unwrap());

    // Get the localization table
    let content = str::from_utf8(file.data.as_ref()).unwrap();
    let table: toml::Table = toml::from_str(content).unwrap();

    // Look up the template string
    let template = table["errors"][&record.key.kind.to_string()]
        .as_str()
        .unwrap_or("Unknown error");
    apply_payload(template, &record.key.payload)
}

fn apply_payload(template: &str, payload: &Payload) -> String {
    match payload {
        Payload::None => template.to_string(),
        Payload::Str(a) => template.replace("{$0}", a),
        Payload::StrPair(a, b) => template.replace("{$0}", a).replace("{$1}", b),
        Payload::StrTriple(a, b, c) => template
            .replace("{$0}", a)
            .replace("{$1}", b)
            .replace("{$2}", c),
        Payload::Num(a) => template.replace("{$0}", &a.to_string()),
        Payload::StrAndNum(a, b) => template.replace("{$0}", a).replace("{$1}", &b.to_string()),
    }
}
