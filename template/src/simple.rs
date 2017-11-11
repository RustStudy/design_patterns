trait Abstract {
    fn format_header(&self) -> String;
    fn format_content(&self, String) -> String;
    fn format_footer(&self) -> String;
    fn build(&self, msg: String) -> String
    {
        self.format_header() + &self.format_content(msg) + &self.format_footer()
    }
}


struct HtmlFormatter;
impl Abstract for HtmlFormatter {
    fn format_header(&self) -> String
    {
        "<!DOCTYPE html>\n<html>\n\t<body>\n".to_string()
    }

    fn format_content(&self, content: String) -> String
    {
        "\t\t".to_string() + &content
    }


    fn format_footer(&self) -> String
    {
        "\n\t</body>\n</html>".to_string()
    }
}

struct MarkdownFormatter;
impl Abstract for MarkdownFormatter {
    fn format_header(&self) -> String
    {
        "# TODO\n".to_string()
    }

    fn format_content(&self, content: String) -> String
    {
        content
    }


    fn format_footer(&self) -> String
    {
        "".to_string()
    }
}


fn main()
{
    let f1 = HtmlFormatter;
    let f2 = MarkdownFormatter;

    println!("{}\n", f1.build("this is content.".to_string()));
    println!("{}", f2.build("this is content.".to_string()));
}