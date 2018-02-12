pub fn parse_x_notation(text: String) -> String {
    text.replace("cx", "ĉ")
        .replace("gx", "ĝ")
        .replace("hx", "ĥ")
        .replace("jx", "ĵ")
        .replace("sx", "ŝ")
        .replace("ux", "ŭ")
}
