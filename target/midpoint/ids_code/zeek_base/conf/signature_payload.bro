##! HTTP requests with blacklisted keywords detection in HTTP.

@load base/frameworks/notice
# @load base/protocols/http

# module HTTP;

export {
    redef enum Notice::Type += {
        ## Indicates that a host sending HTTP requests with URL 
        ## containing blacklisted keywords was detected.
        Bad_Keyword_Signature,
    };
}

event signature_match(state: signature_state, msg: string, data: string) &priority=3
{
    NOTICE([$note=Bad_Keyword_Signature,
            $msg=msg]);
}

