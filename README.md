```rs
//                ____    ______  __  ___  __
//               / __ \  /_  __/ /  |/  / / /
//              / /_/ /   / /   / /|/  / / /
//             / _, _/   / /   / /  / / / /___
//            /_/ |_|   /_/   /_/  /_/ /_____/
//
//   rust macro for generating HTML from a rust-like syntax
```

# Example

```rs
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
```

# Result

```html
<div class="flex justify-center">
  <span class="color-gray-500"> Hello World! </span>
</div>
```

<img width="30%" align="right" src=".github/images/rustacean-flat-happy.png">
