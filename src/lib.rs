use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn getpi(params: i32) -> String {
  let mut ps: i32 = params;
  let mut output: String = "".into();
  let a = 10000;
  let mut b = 0;
  let mut c = 2800;
  let mut d = 0;
  let mut e = 0;
  let mut f = [0; 2801];
  let mut g = 0;
  let mut i: i32;
  for i in 0..c {
    f[i] = a / 5;
  }
  while c != 0 {
    if ps <= 0 {
      break;
    }
    d = 0;
    g = c * 2;
    b = c;
    loop {
      d = d + f[b] * a;
      g -= 1;
      f[b] = d % g;
      d = d / g;
      g -= 1;
      b -= 1;
      if b == 0 {
        break;
      } else {
        d = d * b;
        continue;
      }
    }
    c = c - 14;
    ps -= 4;
    let num = e + d / a;
    if output == "" {
      output += "3.141";
    } else {
      if num < 1000 {
        output += "0";
      } else if num < 100 {
        output += "00";
      }
      output += &num.to_string();
    }
    e = d % a;
  }
  return output;
}
