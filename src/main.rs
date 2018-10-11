use std::ops::Deref;
use std::iter::FromIterator;

trait AsciiCharType{
    fn get_chartype_count(&self)->usize;
}

struct CharVec{
    v:Vec<char>,
}
impl Deref for CharVec{
    type Target=Vec<char>;
    fn deref(&self)-> &Self::Target{
        &self.v
    }
}
impl std::ops::DerefMut for CharVec{
    fn deref_mut(&mut self) -> &mut Self::Target{
        return &mut self.v;
    }
}
// イテレータ周りをVecのものを流用する
impl FromIterator<char> for CharVec{
    fn from_iter<T:IntoIterator<Item=char>>(iter: T) -> Self {
        let mut c = Vec::<char>::new();
        for i in iter{
            c.push(i);
        }
        CharVec::new(c)
    }
}
impl IntoIterator for CharVec{
    type Item = char;
    type IntoIter = ::std::vec::IntoIter<char>;

    fn into_iter(self)->Self::IntoIter{
        self.v.into_iter()
    }
}

impl CharVec{
    fn new(v:Vec<char>)->Self{
        CharVec{
            v:v,
        }
    }
    fn alpha(&self,d:u32)->(usize,usize){
        let d = d as usize;
        if 0x41+5 <= d && d <= 0x5a-5||
            0x61+5 <= d && d <= 0x7a-5
        {
            return (d-5,d+5);
        }
        if d>0x7a-5{return (d-5,0x7a);}
        if d<0x41+5{return (0x41,d+5);}
        if d<0x61+5{return (0x61,d+5);}
        if d>0x5a-5{return (d-5,0x5a);}
        
        (0,0)
    }
}

impl AsciiCharType for CharVec{
    fn get_chartype_count(&self)->usize{
        let mut num=0;
        let mut numeric=false;
        let mut v = (*self).clone();
        let mut hexstring = true;
        v.dedup();
        let mut pmin = 0x00;
        let mut pmax = 0x00;
        for c in v.iter(){
            let d = *c as u32;
            if c.is_ascii_hexdigit()==false{hexstring=false;}
            match d{
                0x20...0x29|0x5b...0x60|0x7b...0x7e=>num+=1,
                0x30...0x39 =>{
                    numeric = true;
                },
                0x41...0x5a|0x61...0x7a=>{
                    let (min,max) = self.alpha(d);
                    if pmin == pmax{
                        pmin = min;
                        pmax = max;
                        continue;
                    }
                    if pmin <= min && min < pmax {
                        pmax = max;
                        continue;
                    }
                    if max <= pmax && pmin < max {
                        pmin = min;
                        continue;
                    }
                    // かぶりがなかったらとりあえず登録
                    num += pmax-pmin+1;
                    // 新しく範囲を決定する。（ソートされているのでこれで問題なし）
                    pmax=max;
                    pmin=min;
                }
                _=>{},
            }
        }
        if hexstring&&!numeric{
            return 16;
        }
        if numeric{
            num+=10;
        }
        if pmax!=pmin{
            num += pmax-pmin+1;
        }
        num
    }
}
use std::env;
use std::io::*;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    let mut arg:Vec<String> = env::args().collect();
    let mut vec:CharVec;
    if arg.len()>=2{
        vec=arg[1].chars().collect();
    }else{
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write("判定対象の文字列を入力してください > ".as_bytes());
        handle.flush();
        input!{s:String};
        vec=s.chars().collect();
    }
    vec.sort();
    let chars = vec.get_chartype_count() as f64;
    let s:String = vec.into_iter().collect();
    println!("文字列： {}",s);
    println!("文字列長： {}",s.len());
    println!("この文字列に含まれる文字の種類(推定値)： {}",chars);
    println!("強度(おおよそのbits数)： {}",chars.powi(s.len() as i32).log2() as u32);
}
