use bytesize::ByteSize;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

pub fn lowercase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param 0 is required for `lowercase` helper."))?;
    let param = param.value().render();
    out.write(&param.to_lowercase())?;
    Ok(())
}

pub fn filesize(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param 0 is required for `filesize` helper."))?;
    if param.value().is_number() {
        let size = param.value().as_u64().ok_or_else(|| {
            RenderError::new("Param to `filesize` helper must be a positive number")
        })?;

        if size > 0 {
            out.write(&ByteSize::b(size).to_string())?;
        }

        Ok(())
    } else {
        dbg!(&param.value());
        Err(RenderError::new(
            "Param to `filesize` helper must be a number",
        ))
    }
}
