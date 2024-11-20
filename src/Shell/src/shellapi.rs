use windows_win::;


static mut V_FUSECONSOLE: bool = false;

pub fn _crtinit() -> int {
    if (!v_fUseConsole) {
        while((filedesc = UU_ropen (TEXT("con"), 0x10002)) == -1) {
            // wait 2 seconds and keep opening console.
            Sleep(2000);
        }
    }
    return 1;
} 

pub fn GetChar () -> int {
    let mut ch: char;
    let mut cnt: UINT;

    if (filedesc != -1)
    {
        do 
        {
            cnt = UU_rread (filedesc, &ch, 1);
            if (0 == cnt)
            {
                /* Yes, we are polling, but let's try not 
                   to take up too much CPU */
                Sleep(1000);
            } else if (-1 == cnt)
        return -1;
        } while (0 == cnt);
        return (int)ch;
    }
    return -1;
}

   


pub fn Gets (TCHAR s, int: cch) -> TCHAR {
    let mut ch: int = -1;
    let mut iIndex: int = 0;
    let mut pch: TCHAR;

    if (v_fUseConsole) {
        v_fUseDebugOut = FALSE;
        return _fgetts (s, cch, stdin);
    }

    pch = s;
    if(cch <= 1)
    {
        return 0;
    }
    while ((iIndex < (cch - 1)) && (ch = GetChar()) != -1) 
    {
        if ((*(pch+iIndex) = (TCHAR)ch) == TEXT('\r')) 
        {
            break;
        }
    iIndex++;
    }
    if (ch == -1)
    return 0;
    *(pch+iIndex) = TEXT('\0');

    return s;
} 



pub fn Puts (s: TCHAR) -> i32 {
    char szAscii [256];
    size_t  nChars;
    
    if (v_fUseConsole) {
        if (v_fUseDebugOut) {
            OutputDebugStringW (s);
            return 1;
        } else {
            return _fputts (s, stdout);
        }
    }

    if ((filedesc != -1)
        && ((nChars = wcstombs (szAscii,s,256)) != (size_t)-1))
    {
        return UU_rwrite (filedesc, (unsigned char *)szAscii, nChars);
    }
        
    return -1;

}
