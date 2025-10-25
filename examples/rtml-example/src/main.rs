use rtml::rtml;

fn main() {
    let html = rtml! {
        Div {
            class: "flex justify-center"

            Span {
                class: "color-gray-500"
                "Hello World!"
            }
        }
    };

    println!("{}", html);
}
