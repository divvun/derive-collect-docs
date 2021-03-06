/// Render some nice human-readable asciidoc files
use crate::collect_docs::{Example, Field, Struct, Type, TypeName};
use std::io::{Result, Write};

pub trait ToAdoc {
    fn write_adoc(&self, w: &mut dyn Write) -> Result<()>;
}

impl ToAdoc for Struct {
    fn write_adoc(&self, w: &mut dyn Write) -> Result<()> {
        writeln!(w, "// Do not edit this file directly!")?;
        writeln!(
            w,
            "// It was generated using derive-collect-docs and will be updated automatically."
        )?;
        writeln!(w)?;
        writeln!(w, "= {}", self.name)?;
        writeln!(w)?;
        writeln!(w, "{}", self.docs)?;
        writeln!(w)?;
        for example in &self.examples {
            example.write_adoc(w)?;
            writeln!(w)?;
        }

        if !self.fields.is_empty() {
            writeln!(w, ".Fields")?;
        }
        for field in &self.fields {
            field.write_adoc(w)?;
        }
        writeln!(w)?;

        Ok(())
    }
}

impl ToAdoc for Field {
    fn write_adoc(&self, w: &mut dyn Write) -> Result<()> {
        write!(w, "* `{}`", self.name)?;
        if self.required {
            writeln!(w, " _(required)_")?;
        } else {
            writeln!(w, " _(optional)_")?;
        }
        writeln!(w, "+")?;
        write!(w, "Type: `")?;
        self.r#type.write_adoc(w)?;
        writeln!(w, "`")?;

        let docs = self.docs.replace("\n\n", "\n+\n");
        if !docs.is_empty() {
            writeln!(w, "+")?;
            write!(w, "{}", docs)?;
        }
        if !self.examples.is_empty() {
            writeln!(w, "+")?;
        }
        for example in &self.examples {
            example.write_adoc(w)?;
        }
        if !self.examples.is_empty() {
            writeln!(w)?;
        }

        Ok(())
    }
}

impl ToAdoc for Example {
    fn write_adoc(&self, w: &mut dyn Write) -> Result<()> {
        writeln!(w, ".Example")?;
        writeln!(w, "[source,{}]", self.lang)?;
        writeln!(w, "----")?;
        writeln!(w, "{}", self.content)?;
        writeln!(w, "----")?;
        Ok(())
    }
}

impl ToAdoc for Type {
    fn write_adoc(&self, mut w: &mut dyn Write) -> Result<()> {
        match self {
            Type::Flat(name) => name.write_adoc(w),
            Type::Nested {
                container_name,
                nested,
            } => {
                container_name.write_adoc(&mut w)?;
                write!(w, "<")?;
                let mut first = true;
                for t in nested {
                    if first {
                        first = false;
                    } else {
                        write!(w, ", ")?;
                    }
                    {
                        t.write_adoc(w)?;
                    }
                }
                write!(w, ">")?;
                Ok(())
            }
        }
    }
}

impl ToAdoc for TypeName {
    fn write_adoc(&self, w: &mut dyn Write) -> Result<()> {
        match self {
            TypeName::Primitive(name) => {
                let name = match name.as_str() {
                    "BTreeMap" => "Map",
                    "HashMap" => "Map",
                    n => n,
                };
                write!(w, "{}", name)
            }
            TypeName::Link(name) => write!(w, "<<{}>>", name),
        }
    }
}
