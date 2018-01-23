# justify.rs

This crate justifies plaintext for display in a terminal emulator in a  (mostly)
Unicode friendly way.

**Examples of use can be found in the file `tests/tests.rs`.**

If the crate is compiled with the `unicode-width` feature (e.g. via `cargo build
--features  unicode-width`), Unicode is handled gracefully. With this feature, a
CJK  character  such as 한 takes two spaces, while combining characters take  0.
Without this feature, every Unicode character takes one space, which can lead to
poor  output  in  some  cases.  If you will only ever  justify  ASCII  text,  or
NFC-normalized Unicode text of Latin languages, you don't need the feature.

The width information is provided by the `wcwidth` crate.

Without `unicode-width` (example text from
[here](https://en.wikipedia.org/wiki/Korea#Etymology)):

```text
"Korea"  is the modern spelling of "Corea", a name attested in English as  early
as  1614.[citation  needed] Korea was transliterated as Cauli in The Travels  of
Marco  Polo,[10] based on the kingdom of Goryeo (Hangul: 고려; Hanja:  高麗;
MR:  Koryŏ), which ruled most of the Korean peninsula during Marco Polo's time.
Korea's  introduction to the West resulted from trade and contact with merchants
from  Arabic  lands,[11]  with  some  records dating back  as  far  as  the  9th
century.[12]  Goryeo's  name  was  a continuation  of  Goguryeo  (Koguryŏ)  the
northernmost  of  the  Three Kingdoms of Korea, which was  officially  known  as
Goryeo  beginning in the 5th century.[13] The original name was a combination of
the  adjective  go ("high, lofty") with the name of a local Yemaek tribe,  whose
original  name  is  thought to have been either *Guru  (溝樓,  "walled  city,"
inferred   from  some  toponyms  in  Chinese  historical  documents)  or  *Gauri
(가우리, "center"). 
```

With `unicode-width` and `wcwidth: true` in `Settings` struct:

```text
"Korea"  is the modern spelling of "Corea", a name attested in English as  early
as  1614.[citation  needed] Korea was transliterated as Cauli in The Travels  of
Marco  Polo,[10] based on the kingdom of Goryeo (Hangul: 고려; Hanja: 高麗;  MR:
Koryŏ),  which  ruled  most of the Korean peninsula during  Marco  Polo's  time.
Korea's  introduction to the West resulted from trade and contact with merchants
from  Arabic  lands,[11]  with  some  records dating back  as  far  as  the  9th
century.[12]  Goryeo's  name  was  a  continuation  of  Goguryeo  (Koguryŏ)  the
northernmost  of  the  Three Kingdoms of Korea, which was  officially  known  as
Goryeo  beginning in the 5th century.[13] The original name was a combination of
the  adjective  go ("high, lofty") with the name of a local Yemaek tribe,  whose
original  name  is  thought  to have been either  *Guru  (溝樓,  "walled  city,"
inferred  from some toponyms in Chinese historical documents) or *Gauri (가우리,
"center").  
```

Notice  that  the  justification is better with `unicode-width`, but  there  are
still  lines where the justification is one off. That's because it's not  always
possible  to  justify perfectly: as Korean characters take two terminal  spaces,
and  Latin  letters  take one, it's possible for there to be an  odd  number  of
characters  on  a line to be justified. Also, depending on your browser, it  may
not look right, try pasting it into a terminal emulator.
