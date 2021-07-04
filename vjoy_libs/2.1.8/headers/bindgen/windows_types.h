// https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types
//typedef int BOOL;
typedef _Bool BOOL;
typedef unsigned char BYTE;
typedef char CHAR;
typedef unsigned long DWORD;
typedef float FLOAT;
typedef int INT;
typedef long LONG;
typedef unsigned long ULONG;
typedef unsigned short USHORT;
typedef unsigned int UINT32;

typedef unsigned int UINT;
typedef void *PVOID;
typedef unsigned char UCHAR;
typedef unsigned short WORD;
typedef short SHORT;
typedef PVOID HANDLE;
typedef PVOID HDEVNOTIFY; // not sure, https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerdevicenotificationa

#define VOID void
#define CALLBACK __stdcall