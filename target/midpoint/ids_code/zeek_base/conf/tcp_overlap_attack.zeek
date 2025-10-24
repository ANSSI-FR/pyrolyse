##! TCP with any payload.

@load base/frameworks/notice
@load base/protocols/http

module tcp;

export {
    redef enum Notice::Type += {
        ## Custom type to display when exporting Zeek payload
        Tcp_content_From_Client,
        Tcp_content_From_Server,
    };
}

event tcp_contents(c: connection, is_orig: bool, seq: count, contents: string) &priority=3
{
    if (is_orig == T)
        {
        NOTICE([$note=Tcp_content_From_Client,
                $msg=contents]);
        }
    else {
        NOTICE([$note=Tcp_content_From_Server,
                $msg=contents]);
    }
    
}
