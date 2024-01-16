use std::path::PathBuf;

use chrono::prelude::*;
use mail_parser::mailbox::maildir::FolderIterator;
use mail_parser::MessageParser;
use mail_parser::MimeHeaders;

const SUFFIXES_TO_EXPORT: &'static [&'static str] = &[".pdf", ".doc"];

struct Attachment {
    name: String,
    subject: String,
    date: chrono::DateTime<Utc>,
    contents: Vec<u8>,
}

impl Attachment {
    pub fn new(
        name: String,
        subject: String,
        date: chrono::DateTime<Utc>,
        contents: Vec<u8>,
    ) -> Self {
        Attachment {
            name: name,
            subject: subject,
            date: date,
            contents: contents,
        }
    }
}

pub struct Extractor {
    maildir: String,
    output_dir: Option<String>,
    since: Option<DateTime<Utc>>,
    suffixes: Vec<String>,
}

impl Extractor {
    pub fn new(maildir: String, output_dir: Option<String>) -> Self {
        Extractor {
            maildir: maildir,
            output_dir: output_dir,
            suffixes: SUFFIXES_TO_EXPORT
                .iter()
                .map(|sfx| String::from(*sfx))
                .collect(),
            since: None,
        }
    }
    pub fn list(&self) -> Result<Vec<String>, String> {
        let mut attachments = Vec::<String>::new();
        self.iterate_over_attachments(|attachment| {
            attachments.push(String::from(attachment.name));
        })?;
        Ok(attachments)
    }

    pub fn extract(&self) -> Result<(), String> {
        self.iterate_over_attachments(|attachment| {
            let out_file = PathBuf::from(self.output_dir.as_ref().unwrap()).join(format!(
                "{}-{}-{}",
                attachment.date.format("%Y-%m-%d"),
                attachment.subject,
                attachment.name
            ));
            if !out_file.exists() {
                println!("Writing to {}...", out_file.to_str().unwrap());
                std::fs::write(out_file, attachment.contents).unwrap();
            }
        })?;
        Ok(())
    }

    fn iterate_over_attachments<F>(&self, mut callback: F) -> Result<(), String>
    where
        F: FnMut(Attachment),
    {
        let maildir_iterator = FolderIterator::new(self.maildir.as_str(), None).map_err(|e| {
            format!(
                "unable to create iterator for maildir '{}': {}",
                self.maildir, e
            )
        })?;

        for folder in maildir_iterator {
            let folder = folder.unwrap();

            for message in folder {
                let message = message.unwrap();
                let parsed_message = &MessageParser::default().parse(message.contents()).unwrap();
                for attachment in parsed_message.attachments() {
                    let attachment_name = attachment.attachment_name().unwrap_or("Untitled");
                    let mut export = false;
                    for sfx in &self.suffixes {
                        if attachment_name.ends_with(sfx.as_str()) {
                            export = true
                        }
                    }
                    if export {
                        callback(Attachment::new(
                            String::from(attachment_name),
                            String::from(parsed_message.subject().unwrap()),
                            chrono::DateTime::from_timestamp(
                                parsed_message.date().unwrap().to_timestamp(),
                                0,
                            )
                            .unwrap(),
                            attachment.contents().to_vec(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extractor_list() {
        let extractor = Extractor::new(String::from("./test/fixtures/simple"), None);
        let attachments = extractor.list().unwrap();
        assert_eq!(1, attachments.len());
    }
    #[test]
    fn test_extractor_extract() {
        let extractor = Extractor::new(
            String::from("./test/fixtures/simple"),
            Some(String::from("./test/tmp_output")),
        );
        let _ = extractor.extract().unwrap();
        assert!(PathBuf::from("./test/tmp_output")
            .join("2024-01-12-test-test.pdf")
            .exists());
    }
}