use crate::{
    config::registry::Color,
    walk::{self, tr::TreeCtxt},
};

pub trait RenderStrategy {
    fn process_file(
        &self,
        ctxt: &mut TreeCtxt,
        file_entry: &walk::fent::FileEntry,
    ) -> anyhow::Result<()>;
    fn process_directory(
        &self,
        ctxt: &mut TreeCtxt,
        file_entry: &walk::fent::FileEntry,
    ) -> anyhow::Result<()>;
}

pub struct TreeRender;
pub struct FileContentRender;

impl RenderStrategy for TreeRender {
    fn process_file(
        &self,
        ctxt: &mut TreeCtxt,
        file_entry: &walk::fent::FileEntry,
    ) -> anyhow::Result<()> {
        // Same as your `handle_file` logic
        ctxt.buf
            .print_file(file_entry, &ctxt.path_builder, ctxt.rg.entries().file())?;
        ctxt.buf.newline()?;
        Ok(())
    }

    fn process_directory(
        &self,
        ctxt: &mut TreeCtxt,
        file_entry: &walk::fent::FileEntry,
    ) -> anyhow::Result<()> {
        // Same as your `handle_directory` logic
        ctxt.rg.blue(ctxt.buf)?;
        ctxt.buf
            .print_dir(file_entry, &ctxt.path_builder, ctxt.rg.entries().dir())?;
        ctxt.rg.reset(ctxt.buf)?;
        ctxt.buf.newline()?;
        Ok(())
    }
}

impl RenderStrategy for FileContentRender {
    fn process_file(
        &self,
        ctxt: &mut TreeCtxt,
        file_entry: &walk::fent::FileEntry,
    ) -> anyhow::Result<()> {
        // Print the file's content instead of the file name
        let file_path = file_entry.absolute_path();
        let content = std::fs::read_to_string(file_path)?;
        ctxt.buf.write_message(&content)?;
        ctxt.buf.newline()?;
        Ok(())
    }

    fn process_directory(
        &self,
        ctxt: &mut TreeCtxt,
        file_entry: &walk::fent::FileEntry,
    ) -> anyhow::Result<()> {
        // Optionally handle directories differently, or just ignore them
        ctxt.buf.write_message("Skipping directory\n")?;
        ctxt.buf.newline()?;
        Ok(())
    }
}
