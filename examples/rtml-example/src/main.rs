use rtml::rtml;

fn main() {
    let html = rtml! {
        Div {
            class: "flex justify-center"

            Span {
                class: "text-pink-800 p-4"
                "Hello World!"
            }
        }
    };

    println!("{}", html);
}
