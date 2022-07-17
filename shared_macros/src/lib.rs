//use miniserde::{json, Deserialize};
use std::str::FromStr;
use core::fmt::Debug;
pub use log;
pub use colored;
pub use linereader;

pub fn find_arg<T>(argname: &str) -> Option<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug
{
    std::env::args()
        .position(|a| a == argname)
        .map(|p| std::env::args().nth(p + 1))
        .flatten()
        .map(|s| s.parse::<T>().expect(&format!("Was unable to parse {}", argname)))
}

pub fn find_flag(flagname: &str) -> bool {
    let arg = std::env::args().position(|a| a == flagname);
    match arg {
        None => false,
        Some(_) => true,
    }
}

#[macro_export]
macro_rules! tool {
    (args: $(- $identname:ident $(: $typeof:ty )? $( = $default:expr)?; $( ? $cond:expr )? $( => $lit:literal )? )+; body: $main_body:expr) => {
        use colored::*;
        let case1 = (|| {
            if (find_flag("-h") || find_flag("-H") || find_flag("--help") || find_flag("help") || find_flag("-help") ) {
                return Err("Here's some help!".to_string());
            }
            $(
                let argflag = format!("--{}", stringify!($identname));
                tool!(- $identname argflag $(: $typeof )? $( = $default)?; $( ? $cond )? $( => $lit )?);
            )+;
            if 1 != 1 {
                return Err("The universe is broken".to_string());
            }
            Ok(($main_body)())
        })();

        match case1 {
            Err(e1) => {
                eprintln!("Error: {:#?}", e1);
                eprintln!("⚘⚘⚘ Help: ⚘⚘⚘\n");
                $(
                    eprintln!("--{} ⚘", stringify!($identname).magenta());
                    $(
                        eprintln!("  type: {}", stringify!($typeof).bold().italic().magenta());
                    )? 
                    $( 
                        eprintln!("  default: {}", stringify!($default).cyan().bold());
                    )?
                    $( 
                        eprintln!("  fails if: {}", stringify!($cond).green());
                    )?
                    $( 
                        eprintln!("      because: {}", $lit.green()); 
                    )?
                    eprintln!("");
                )+;
            }
            _ => {}
        };
    };
    (- $argname:ident $argflag:ident : $typ:ty; $($remaining_tokens:tt)*) => {
        let $argname = find_arg::<$typ>(&$argflag).ok_or_else(|| format!("{} is a required argument", $argflag))?;
        tool!($($remaining_tokens)*);
    };
    (- $argname:ident $argflag:ident : $typ:ty = $default:expr; $($remaining_tokens:tt)*) => {
        let $argname = find_arg::<$typ>(&$argflag).unwrap_or_else(|| $default);
        tool!($($remaining_tokens)*);
    };
    (- $argname:ident $argflag:ident; $($remaining_tokens:tt)*) => {
        let $argname = find_flag(&$argflag);
        tool!($($remaining_tokens)*);
    };
    (? $evalfn:expr => $err:literal) => {
        if ($evalfn) {
            return Err($err.to_string());
        }
    };
    (? $evalfn:expr) => {
        if ($evalfn) {
            return Err(stringify!($evalfn).to_string());
        }
    };
    () => {};
}

// todo: create another macro using mpsc sync and linereader for those that don't require transformations of the original string, i.e ones that can just operate on byte-slices.

/*#[macro_export]
macro_rules! readin {
    ($tx:ident, $closure:expr) => {
        use std::io::prelude::*;
        use corosensei::{Coroutine, CoroutineResult, Yielder};
        //use linereader::LineReader;
        //use std::thread;
        //use std::sync::mpsc::sync_channel;
        let mut reader = std::io::BufReader::new(std::io::stdin());
        let mut writer = std::io::BufWriter::new(std::io::stdout().lock());
        //let ($tx, rx) = sync_channel(1000);

        let mut coroutine = Coroutine::new(move |$tx, _| {
            let mut line = String::new(); 
            while reader.read_line(&mut line)? > 0 {
                ($closure)(line.as_str());
                line.clear();
            };
            Ok(()) as Result<(), std::io::Error>
        });

        loop {
            match coroutine.resume(0) {
                CoroutineResult::Yield(msg) => {
                    let res = writer.write_all(&msg.as_bytes());
                    let _ = writer.write_all(b"\n");
                    if res.is_err() {
                        break;
                    }
                },
                CoroutineResult::Return(_) => break,
            };
        }
    }
}*/

#[macro_export]
macro_rules! filter_in {
    ($closure:expr) => {
        use std::io::prelude::*;
        use linereader::LineReader;
        let mut reader = LineReader::new(std::io::stdin().lock());
        let mut writer = std::io::BufWriter::new(std::io::stdout().lock());

        let _ = reader.for_each(|line| {
            if ($closure)(line) {
                let res = writer.write_all(line);
                //let _ = writer.write_all(b"\n");
                if res.is_err() {
                    return Ok(false);
                }
            }
            Ok(true)
        });
    }
}

#[macro_export]
macro_rules! readin {
    ($writer:ident, $closure:expr) => {
        use std::io::prelude::*;
        use linereader::LineReader;
        let mut reader = LineReader::new(std::io::stdin().lock());
        let mut $writer = std::io::BufWriter::new(std::io::stdout().lock());

        let _ = reader.for_each(|line| {
            ($closure)(line);
            Ok(true)
        });
    }
}

// give ownership of the line? to avoid repeated allocation? or use a stackful generator to allow for yielding unowned data?

#[macro_export]
macro_rules! then {
    ($ex:expr, |$arg:ident| $($closure:expr);+) => {
        $ex
        $(
            .and_then(|$arg| $closure)
        )+
    }
}

#[macro_export]
macro_rules! chan {
    ($txname:ident, $rxname:ident) => {
        let ($txname, $rxname) = async_std::channel::unbounded();
    };
}

#[macro_export]
macro_rules! spawn {
    ($($cloned_obj:ident),* => $($task:tt)+) => {
        {
            $(
                let $cloned_obj = $cloned_obj.clone();
            )*
            async_std::task::spawn(async move{
                    $($task)+
            });
        }
    }
}

#[macro_export]
macro_rules! whileok {
    ($tx:ident => $msgname:ident { $($task:tt)+ }) => {
        while let Ok($msgname) = $tx.recv().await { $($task)+ }
    }
}

#[macro_export]
macro_rules! senditer {
    ($iter:expr => $txname:ident => $rxname:ident) => {
        let ($txname, $rxname) = async_std::channel::unbounded();
        let __tx_cloned = $txname.clone();
        spawn!( =>
            for __item in $iter {
                let _ = __tx_cloned.send(__item).await;
            }
        );
    }
}

#[macro_export]
macro_rules! pipe {
    ($($num_workers:literal * $source_channel:ident => $closure:expr => $txname:ident => $rxname:ident),+) => {
        $(
            let ($txname, $rxname) = async_std::channel::unbounded();
            for _ in (0..($num_workers)) {
                let __rx = $source_channel.clone();
                let new_tx = $txname.clone();
                async_std::task::spawn(async move {
                    loop {
                        while let Ok(item) = __rx.recv().await {
                            if let Some(result) = ($closure)(item).await {
                                let _ = new_tx.send(result).await;
                            }
                        }
                    }
                });
            }
        )+
    };
}
