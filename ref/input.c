// for refrence, attempt to replicate in rust
#include <windows.h>

LRESULT CALLBACK LowLevelKeyboardProc( int nCode, WPARAM wParam, LPARAM lParam )
{
    char p_key;
    int send;
    KBDLLHOOKSTRUCT *p_kbdllhookstruct = (KBDLLHOOKSTRUCT *)lParam;
    switch( wParam )
    {
        case WM_KEYUP:
        {
            p_key = p_kbdllhookstruct->vkCode;
            send = 1;
            break;
        }
        case WM_KEYDOWN:
        {
            p_key = p_kbdllhookstruct->vkCode;
            send = 1;
            break;
        }
        default:
        {
            send = 0;
            break;
        }
    }
    if( send )
    {
        PostMessageA(NULL, wParam, p_key, 0 );
    }
    return CallNextHookEx( NULL, nCode, wParam, lParam );
}

int LowLevelKeyboardProc_install( )
{
    return SetWindowsHookEx( WH_KEYBOARD_LL, LowLevelKeyboardProc, GetModuleHandle(NULL), 0 );
}

int main() {
    LowLevelKeyboardProc_install();
    MSG msg;
    while( GetMessageA( &msg, NULL, 0, 0 ) )
    {
        TranslateMessage( &msg );
        DispatchMessageA( &msg );
        printf("%d\n", msg.wParam);
    }
}