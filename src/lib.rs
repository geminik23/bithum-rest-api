#[macro_use] extern crate log;
extern crate serde;
#[macro_use] extern crate serde_json;
extern crate chrono;
extern crate anyhow;


mod authenticate;

pub mod rest;
pub mod ws;

// public static bithumbApiResponseCodeToString(codeStr: string): string {
//     switch (codeStr) {
//       case '5100':
//         return 'Bad Request';
//       case '5200':
//         return 'Not Member';
//       case '5300':
//         return 'Invalid Apikey';
//       case '5302':
//         return 'Method Not Allowed';
//       case '5400':
//         return 'Database Fail';
//       case '5500':
//         return 'Invalid Parameter';
//       case '5600':
//         return 'Output a contextual message';
//       case '5900':
//         return 'Unknown Error';
//       case '0000':
//         return 'Success';
//       default:
//         return 'Unknown';
//     }
//   }
