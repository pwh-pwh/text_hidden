# Text Hidden

A hidden text field that can be used to hide text in a form.
Can be used for some social software to hide and encrypt text communication, such as WeChat, QQ, and so on.

## Usage
todo

## Example
```rust
fn main() {
    //加密方法
    let cipher = NoCipher;
    //文本水印摆放位置
    let pose = SimplePose::default();
    let text_hidden = TextHidden::new(cipher, pose, '\u{200B}', '\u{200C}');
    let th = text_hidden.text_hidden("hello", "key");
    println!("加密后:{}长度:{}!", th,th.len());
    let result = text_hidden.text_recover(th.as_str());
    let result_str = result.unwrap();
    println!("解密后:{}长度:{}", result_str, result_str.len());
}
```
效果
```text
加密后:hello​﻿‌﻿‌﻿​﻿‌﻿​﻿‌﻿‌﻿‍﻿​﻿‌﻿‌﻿​﻿​﻿‌﻿​﻿‌﻿‍﻿​﻿‌﻿‌﻿‌﻿‌﻿​﻿​﻿‌长度:158!
解密后:key长度:3

```