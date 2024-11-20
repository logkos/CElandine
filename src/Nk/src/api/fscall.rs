#![allow(non_snake_case)]

pub enum FSAPI {

}

pub enum FSYSAPI {
FINDFIRSTFILE = 8, // FindFirstFileW
CREATEFILE = 9, // CreateFileW
REGCLOSE = 17,  // RegCloseKey
EGCREATEEX = 18,          // RegCreateKeyEx
REGOPENEX =  23,         // RegOpenKeyEx
REGQUERYEX =  25,          // RegQueryValueEx
REGSETEX = 26,          // RegSetValueEx
REGFLUSH = 49,          // RegCloseKey
NOTIFYFS = 50,          // NotifyMountedFS
REGTESTSET = 134,         // RegTestSetValue
OPENMODULE = 141
}