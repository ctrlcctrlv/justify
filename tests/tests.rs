extern crate justify;
use justify::{Settings, justify, justify_paragraph, InsertAt};
#[test]
fn less_than_width() {
    let settings = Settings::default();
    assert_eq!(justify("E e", &settings), "E e");
}

#[test]
fn long_text() {
    let settings = Settings::default();
    let justified = "If  a program contains arithmetic overflow, the programmer has made an error. In
the  following discussion, we maintain a distinction between arithmetic overflow
and   wrapping  arithmetic.  The  first  is  erroneous,  while  the  second   is
intentional.";
    let plain = "If a program contains arithmetic overflow, the programmer has made an error. In the following discussion, we maintain a distinction between arithmetic overflow and wrapping arithmetic. The first is erroneous, while the second is intentional.";
    assert_eq!(justify(plain, &settings), justified);
}

#[test]
fn word_longer_than_width_hyphenate() {
    let settings = Settings { width: 10, hyphenate_overflow: true, ..Settings::default() };
    let plain = "123456 789 1234567890 123456789012 123 456 89";
    let justified = "123456 789\n1234567890\n123456789-\n012    123\n456 89";
    assert_eq!(justify(plain, &settings), justified);
}

#[test]
fn word_longer_than_width() {
    let settings = Settings { width: 10, ..Settings::default() };
    let plain = "123456 789 1234567890 123456789012 123 456 89";
    let justified = "123456 789\n1234567890\n123456789012\n123 456 89";
    assert_eq!(justify(plain, &settings), justified);
}

#[test]
fn multi_hyphenate() {
    let settings = Settings { width: 4, hyphenate_overflow: true, ..Settings::default() };
    let plain = "Supercalifragilisticexpialidocious";
    let justified = "Sup-\nerc-\nali-\nfra-\ngil-\nist-\nice-\nxpi-\nali-\ndoc-\niou-\ns";
    assert_eq!(justify(plain, &settings), justified);
}

#[test]
#[should_panic(expected = "Expected `text` to contain no newlines but it did")]
fn justify_paragraph_with_newline() {
    let settings = Settings::default();
    let plain = "If a program contains arithmetic overflow, the programmer has made an error. In the following discussion, we maintain a distinction between arithmetic overflow and wrapping arithmetic. The first is erroneous, while the second is intentional.\n\nIf a program contains arithmetic overflow, the programmer has made an error. In the following discussion, we maintain a distinction between arithmetic overflow and wrapping arithmetic. The first is erroneous, while the second is intentional.";
    justify_paragraph(plain, &settings);
}

#[test]
fn justify_lipsum_right() {
    let settings = Settings { insert_at: InsertAt::Right, ..Settings::default() };
    let plain = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus viverra tempor dolor vitae tempus. Duis imperdiet faucibus magna sed convallis. In ullamcorper a quam eu blandit. Aenean sagittis sit amet risus at condimentum. Integer venenatis a turpis a porttitor. Aliquam eu justo nec metus egestas suscipit eu sed libero. Mauris a ultrices tortor. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Nunc fringilla tempor pellentesque. Pellentesque facilisis mi eu condimentum interdum. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Mauris consequat luctus condimentum. Sed eget purus elit. Curabitur pretium elementum enim, gravida sodales neque mollis ut. Curabitur malesuada euismod fermentum.";
    let justified = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus viverra  tempor
dolor vitae tempus. Duis imperdiet faucibus magna sed convallis. In  ullamcorper
a quam eu blandit. Aenean  sagittis  sit  amet  risus  at  condimentum.  Integer
venenatis a turpis a porttitor. Aliquam eu justo nec metus egestas  suscipit  eu
sed libero. Mauris a ultrices tortor. Vestibulum ante ipsum primis  in  faucibus
orci  luctus  et  ultrices  posuere  cubilia  Curae;   Nunc   fringilla   tempor
pellentesque. Pellentesque facilisis mi eu  condimentum  interdum.  Orci  varius
natoque penatibus et magnis  dis  parturient  montes,  nascetur  ridiculus  mus.
Mauris consequat luctus condimentum. Sed  eget  purus  elit.  Curabitur  pretium
elementum enim, gravida sodales neque mollis  ut.  Curabitur  malesuada  euismod
fermentum.";
    assert_eq!(justify(plain, &settings), justified);
}

#[test]
fn empty_string_justify() {
    let settings = Settings::default();
    assert_eq!(justify("", &settings), "");
}

#[test]
fn one_line_justify() {
    let settings = Settings::default();
    assert_eq!(justify("Questions are  good", &settings), "Questions are good");
}

#[test]
fn many_empty_lines_justify() {
    let settings = Settings { width: 9, ..Settings::default() };
    assert_eq!(justify("\n\n\n\n\n\n\nQuestions are  good\n\n\n      \n\n\n", &settings), "Questions\nare good");
}

#[cfg(feature="unicode-width")]
#[test]
fn test_unicode_cjk_justify() {
    let settings = Settings { wcwidth: true, ..Settings::default() };
    let plain = "Lorem ipsum dolor 新しく作成した sit amet, consectetur adipiscing elit. Vivamus 機能をテストするために viverra tempor dolor vitae tempus. CJK （中国語、日本語または韓国語） Duis imperdiet faucibus magna sed convallis. In ullamcorper a quam eu blandit. Aenean sagittis sit amet risus at condimentum. テ Integer キ venenatis ス a turpis a porttitor. ト Aliquam eu justo nec metus が無作為に必要ですが、 egestas suscipit eu sed libero. Mauris a ultrices tortor. 日本語 Vestibulum わかりません。 ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Nunc fringilla tempor pellentesque. Pellentesque facilisis mi eu condimentum interdum. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Mauris consequat luctus condimentum. Sed eget purus elit. Curabitur pretium elementum enim, gravida sodales neque mollis ut. Curabitur malesuada euismod fermentum.";
    let justified = "Lorem  ipsum dolor 新しく作成した sit amet, consectetur adipiscing elit. Vivamus
機能をテストするために     viverra    tempor    dolor    vitae    tempus.    CJK
（中国語、日本語または韓国語）     Duis    imperdiet    faucibus    magna    sed
convallis.  In ullamcorper a quam eu blandit. Aenean sagittis sit amet risus  at
condimentum.  テ  Integer  キ venenatis ス a turpis a porttitor. ト  Aliquam  eu
justo  nec metus が無作為に必要ですが、 egestas suscipit eu sed libero. Mauris a
ultrices  tortor. 日本語 Vestibulum わかりません。 ante ipsum primis in faucibus
orci   luctus   et  ultrices  posuere  cubilia  Curae;  Nunc  fringilla   tempor
pellentesque.  Pellentesque  facilisis mi eu condimentum interdum.  Orci  varius
natoque  penatibus  et  magnis dis parturient montes,  nascetur  ridiculus  mus.
Mauris  consequat  luctus  condimentum. Sed eget purus elit.  Curabitur  pretium
elementum  enim,  gravida sodales neque mollis ut. Curabitur  malesuada  euismod
fermentum.";
    assert_eq!(justify(plain, &settings), justified);
}

#[cfg(feature="unicode-width")]
#[test]
fn test_unicode_cjk_only() {
    let settings = Settings { hyphenate_overflow: true, hyphen: "", width: 10, wcwidth: true, ..Settings::default() };
    let plain = "テストテストテストテストテAAスAAトテストテストテストテスト";
    let justified = "テストテス\nトテストテ\nストテAAス\nAAトテスト\nテストテス\nトテスト";
    let res = justify(plain, &settings);
    assert_eq!(res, justified);
}

#[cfg(feature="unicode-width")]
#[test]
/// This test is from https://ja.wikipedia.org/wiki/Rust_(%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E)
fn japanese_justify() {
    let settings = Settings { ignore_spaces: true, hyphenate_overflow: true, hyphen: "", wcwidth: true, ..Settings::default() };
    let plain = "ネットワークを相手に動作する比較的大きな、サーバやクライアントプログラムを作成するのに都合が良い言語を目指したものである。結果として、安全性、メモリ管理、並列性が、この言語の目立った特徴となっている。性能はC++言語に匹敵するものになるはずである[14]。

バージョン1.12より導入されたMIR (Mid-level IR)[15] によって、コンパイルと実行時間の迅速化ならびに型チェックの正確性の実現が図られている。ブロックに中括弧を使うなど、構文はC言語風である。

制御構造には if, else, do, while, for などがある。以上のようにC言語風であるが、C言語のキーワードが全てあるわけではなく、一方で多方向分岐の match 文など、あまり馴染みがないキーワードもある[16]。

構文は似ているが、意味論（セマンティクス）では大きく異なる部分がある。 このシステムの設計はメモリー・セーフであり、ヌルポインタや不正なメモリ域を指すポインターは許容されていない。データの値は決まったフォームのみで初期化され、それらの全ての入力は既に初期化されている必要がある[17]。

この言語の型システムではHaskell言語に倣い「型クラス」を用いることができる。これはアドホックな多相性を容易にするものであり、可変型宣言により実現されるものである。高類多相性[18]など、Haskell言語にある他の特徴はサポートされていない。

Rust言語では予約語「let」で宣言された変数に対して型推論が行われる。これらの変数は型を決定するための値を必要としない。コード中のどこかでそれらの変数への値の代入[19]が失敗した場合にはコンパイル時エラーが発生する[20]。型が明示された関数の引数に対しては型推論は行われない。";
    let justified = "ネットワークを相手に動作する比較的大きな、サーバやクライアントプログラムを作成す
るのに都合が良い言語を目指したものである。結果として、安全性、メモリ管理、並列性
が、この言語の目立った特徴となっている。性能はC++言語に匹敵するものになるはずで
ある[14]。

バージョン1.12より導入されたMIR (Mid-level IR)[15] によって、コンパイルと実行時
間の迅速化ならびに型チェックの正確性の実現が図られている。ブロックに中括弧を使う
など、構文はC言語風である。

制御構造には if, else, do, while, for などがある。以上のようにC言語風であるが、C
言語のキーワードが全てあるわけではなく、一方で多方向分岐の match 文など、あまり
馴染みがないキーワードもある[16]。

構文は似ているが、意味論（セマンティクス）では大きく異なる部分がある。 このシス
テムの設計はメモリー・セーフであり、ヌルポインタや不正なメモリ域を指すポインター
は許容されていない。データの値は決まったフォームのみで初期化され、それらの全ての
入力は既に初期化されている必要がある[17]。

この言語の型システムではHaskell言語に倣い「型クラス」を用いることができる。これ
はアドホックな多相性を容易にするものであり、可変型宣言により実現されるものである
。高類多相性[18]など、Haskell言語にある他の特徴はサポートされていない。

Rust言語では予約語「let」で宣言された変数に対して型推論が行われる。これらの変数
は型を決定するための値を必要としない。コード中のどこかでそれらの変数への値の代入
[19]が失敗した場合にはコンパイル時エラーが発生する[20]。型が明示された関数の引数
に対しては型推論は行われない。";
    let res = justify(plain, &settings);
    assert_eq!(res, justified);
}

#[cfg(feature="unicode-width")]
#[test]
/// This test is from https://th.wikipedia.org/wiki/%E0%B8%AA%E0%B8%99%E0%B8%B4%E0%B8%A1
fn thai_justify() {
    //let settings = Settings { wcwidth: true, ..Settings::default() };
    let settings = Settings { ignore_spaces: true, hyphenate_overflow: true, hyphen: "", wcwidth: true, ..Settings::default() };
    let plain = "เป็นปฏิกิริยาที่พบเห็นได้ง่ายๆ กับสิ่งก่อสร้างต่าง ๆ ที่มีเหล็กเป็นองค์ประกอบ แต่เป็นปฏิกิริยาที่เกิดขึ้นอย่างช้าๆ อาจจะกินเวลายาวนาน เกิดขึ้นเมื่อมีเหล็กสัมผัสกับน้ำและความชื้น โดยจะค่อย ๆ สึกกร่อน กลายเป็นเหล็กออกไซด์ มีชื่อทางเคมีว่า ไฮเดรตเฟอริกออกไซด์ หรือที่เรารู้จักกันว่า สนิมเหล็ก (Fe2O3.XH2O3) มีลักษณะเป็นคราบสีแดง ซึ่งไม่สามารถเกาะอยู่บนผิวของเหล็กได้อย่างเหนียวแน่น สามารถหลุดออกออกไปได้ง่าย ทำให้เนื้อเหล็กที่อยู่ชั้นในสามารถเกิดสนิมต่อจนกระทั่งหมดทั้งชิ้น กระบวนการเกิดสนิมค่อนข้างซับซ้อน โดยมีปัจจัยคือ น้ำและออกซิเจน ซึ่งมีอยู่ทั่วไปในบรรยกาศโลก เหล็กจะเกิดสนิมเร็วขึ้นในบางสภาวะ เช่น สภาวะที่เป็นกรด ตามชายทะเลที่ไอเกลือเข้มข้น เป็นต้น";
    let justified = "เป็นปฏิกิริยาที่พบเห็นได้ง่ายๆ กับสิ่งก่อสร้างต่าง ๆ ที่มีเหล็กเป็นองค์ประกอบ แต่เป็นปฏิกิริยาที่เกิดขึ้นอย่างช้า
ๆ อาจจะกินเวลายาวนาน เกิดขึ้นเมื่อมีเหล็กสัมผัสกับน้ำและความชื้น โดยจะค่อย ๆ สึกกร่อน กลายเป็นเหล็ก
ออกไซด์ มีชื่อทางเคมีว่า ไฮเดรตเฟอริกออกไซด์ หรือที่เรารู้จักกันว่า สนิมเหล็ก (Fe2O3.XH2O3) มีลักษณะ
เป็นคราบสีแดง ซึ่งไม่สามารถเกาะอยู่บนผิวของเหล็กได้อย่างเหนียวแน่น สามารถหลุดออกออกไปได้ง่าย ทำใ
ห้เนื้อเหล็กที่อยู่ชั้นในสามารถเกิดสนิมต่อจนกระทั่งหมดทั้งชิ้น กระบวนการเกิดสนิมค่อนข้างซับซ้อน โดยมีปัจจัยคื
อ น้ำและออกซิเจน ซึ่งมีอยู่ทั่วไปในบรรยกาศโลก เหล็กจะเกิดสนิมเร็วขึ้นในบางสภาวะ เช่น สภาวะที่เป็นกร
ด ตามชายทะเลที่ไอเกลือเข้มข้น เป็นต้น";
    let res = justify(plain, &settings);
    assert_eq!(res, justified);
}
