use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_to_clipboard() {
        let mut ctx = ClipboardContext::new().unwrap();
        let to_copy = "First";
        ctx.set_contents(to_copy.into()).unwrap();

        assert_eq!(ctx.get_contents().unwrap(), to_copy.to_string());
    }


}
