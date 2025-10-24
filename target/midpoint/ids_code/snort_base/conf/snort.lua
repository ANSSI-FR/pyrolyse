---------------------------------------------------------------------------
-- Snort++ configuration
---------------------------------------------------------------------------

-- there are over 200 modules available to tune your policy.
-- many can be used with defaults w/o any explicit configuration.
-- use this conf as a template for your specific configuration.

-- 1. configure defaults
-- 2. configure inspection
-- 3. configure bindings
-- 4. configure performance
-- 5. configure detection
-- 6. configure filters
-- 7. configure outputs
-- 8. configure tweaks

---------------------------------------------------------------------------
-- 1. configure defaults
---------------------------------------------------------------------------

-- HOME_NET and EXTERNAL_NET must be set now
-- setup the network addresses you are protecting
HOME_NET = 'any'

-- set up the external network addresses.
-- (leave as "any" in most situations)
EXTERNAL_NET = 'any'

include 'snort_defaults.lua'

---------------------------------------------------------------------------
-- 2. configure inspection
---------------------------------------------------------------------------

-- mod = { } uses internal defaults
-- you can see them with snort --help-module mod

-- mod = default_mod uses external defaults
-- you can see them in snort_defaults.lua

-- the following are quite capable with defaults:

stream = {  }

stream_ip_0 =
{
    policy = 'linux',
}

stream_ip_1 =
{
    policy = 'windows',
}

stream_ip_2 =
{
    policy = 'bsd',
}

stream_ip_3 =
{
    policy = 'bsd_right',
}

stream_ip_4 =
{
    policy = 'solaris',
}

stream_ip_5 =
{
    policy = 'last',
}

stream_ip_6 =
{
    policy = 'first',
}

stream_ip_7 = { }


stream_tcp_0 =
{
    policy = 'linux',
}

stream_tcp_1 =
{
    policy = 'windows',
}

stream_tcp_2 =
{
    policy = 'bsd',
}

stream_tcp_4 =
{
    policy = 'solaris',
}

stream_tcp_5 =
{
    policy = 'last',
}

stream_tcp_6 =
{
    policy = 'first',
}

stream_tcp_7 = { }

stream_tcp_8 =
{
    policy = 'old_linux',
}

stream_tcp_9 =
{
    policy = 'macos',
}

stream_tcp_10 =
{
    policy = 'irix',
}

stream_tcp_11 =
{
    policy = 'hpux10',
}

stream_tcp_12 =
{
    policy = 'hpux11',
}

stream_tcp_13 =
{
    policy = 'win_2003',
}

stream_tcp_14 =
{
    policy = 'vista',
}

stream_tcp_15 =
{
    policy = 'proxy',
}


stream_icmp = { }
stream_udp = { }
stream_user = { }
stream_file = { }

arp_spoof = { }
back_orifice = { }
dns = { }
imap = { }
netflow = {}
normalizer = { }
pop = { }
rpc_decode = { }
sip = { }
ssh = { }
ssl = { }
telnet = { }

cip = { }
dnp3 = { }
iec104 = { }
mms = { }
modbus = { }
s7commplus = { }

dce_smb = { }
dce_tcp = { }
dce_udp = { }
dce_http_proxy = { }
dce_http_server = { }

-- see snort_defaults.lua for default_*
gtp_inspect = default_gtp
port_scan = default_med_port_scan
smtp = default_smtp

ftp_server = default_ftp_server
ftp_client = { }
ftp_data = { }

http_inspect = { }
http2_inspect = { }

-- see file_magic.rules for file id rules
file_id = { rules_file = 'file_magic.rules' }
file_policy = { }

js_norm = default_js_norm

-- the following require additional configuration to be fully effective:

appid =
{
    -- appid requires this to use appids in rules
    --app_detector_dir = 'directory to load appid detectors from'
}

--[[
reputation =
{
    -- configure one or both of these, then uncomment reputation
    -- (see also related path vars at the top of snort_defaults.lua)

    --blacklist = 'blacklist file name with ip lists'
    --whitelist = 'whitelist file name with ip lists'
}
--]]

---------------------------------------------------------------------------
-- 3. configure bindings
---------------------------------------------------------------------------

wizard = default_wizard

binder =
{
    -- port bindings required for protocols without wizard support
    { when = { proto = 'tcp', nets = '[192.168.10.79]'}, use = { type = 'stream_tcp', name = 'stream_tcp_15'} },
    { when = { proto = 'tcp', nets = '[192.168.10.78]'}, use = { type = 'stream_tcp', name = 'stream_tcp_14'} },
    { when = { proto = 'tcp', nets = '[192.168.10.77]'}, use = { type = 'stream_tcp', name = 'stream_tcp_13'} },
    { when = { proto = 'tcp', nets = '[192.168.10.76]'}, use = { type = 'stream_tcp', name = 'stream_tcp_12'} },
    { when = { proto = 'tcp', nets = '[192.168.10.75]'}, use = { type = 'stream_tcp', name = 'stream_tcp_11'} },
    { when = { proto = 'tcp', nets = '[192.168.10.74]'}, use = { type = 'stream_tcp', name = 'stream_tcp_10'} },
    { when = { proto = 'tcp', nets = '[192.168.10.73]'}, use = { type = 'stream_tcp', name = 'stream_tcp_9'} },
    { when = { proto = 'tcp', nets = '[192.168.10.72]'}, use = { type = 'stream_tcp', name = 'stream_tcp_8'} },
    { when = { proto = 'tcp', nets = '[192.168.10.71]'}, use = { type = 'stream_tcp', name = 'stream_tcp_7'} },
    { when = { proto = 'tcp', nets = '[192.168.10.70]'}, use = { type = 'stream_tcp', name = 'stream_tcp_6'} },
    { when = { proto = 'tcp', nets = '[192.168.10.69]'}, use = { type = 'stream_tcp', name = 'stream_tcp_5'} },
    { when = { proto = 'tcp', nets = '[192.168.10.68]'}, use = { type = 'stream_tcp', name = 'stream_tcp_4'} },
    { when = { proto = 'tcp', nets = '[192.168.10.66]'}, use = { type = 'stream_tcp', name = 'stream_tcp_2'} },
    { when = { proto = 'tcp', nets = '[192.168.10.65]'}, use = { type = 'stream_tcp', name = 'stream_tcp_1'} },
    { when = { proto = 'tcp', nets = '[192.168.10.64]'}, use = { type = 'stream_tcp', name = 'stream_tcp_0'} },



    { when = { proto = 'udp', ports = '53', role='server' },  use = { type = 'dns' } },
    { when = { proto = 'tcp', ports = '53', role='server' },  use = { type = 'dns' } },
    { when = { proto = 'tcp', ports = '111', role='server' }, use = { type = 'rpc_decode' } },
    { when = { proto = 'tcp', ports = '502', role='server' }, use = { type = 'modbus' } },
    { when = { proto = 'tcp', ports = '2123 2152 3386', role='server' }, use = { type = 'gtp_inspect' } },
    { when = { proto = 'tcp', ports = '2404', role='server' }, use = { type = 'iec104' } },
    { when = { proto = 'udp', ports = '22222', role = 'server' }, use = { type = 'cip' } },
    { when = { proto = 'tcp', ports = '44818', role = 'server' }, use = { type = 'cip' } },

    { when = { proto = 'tcp', service = 'dcerpc' },  use = { type = 'dce_tcp' } },
    { when = { proto = 'udp', service = 'dcerpc' },  use = { type = 'dce_udp' } },
    { when = { proto = 'udp', service = 'netflow' }, use = { type = 'netflow' } },

    { when = { service = 'netbios-ssn' },      use = { type = 'dce_smb' } },
    { when = { service = 'dce_http_server' },  use = { type = 'dce_http_server' } },
    { when = { service = 'dce_http_proxy' },   use = { type = 'dce_http_proxy' } },

    { when = { service = 'cip' },              use = { type = 'cip' } },
    { when = { service = 'dnp3' },             use = { type = 'dnp3' } },
    { when = { service = 'dns' },              use = { type = 'dns' } },
    { when = { service = 'ftp' },              use = { type = 'ftp_server' } },
    { when = { service = 'ftp-data' },         use = { type = 'ftp_data' } },
    { when = { service = 'gtp' },              use = { type = 'gtp_inspect' } },
    { when = { service = 'imap' },             use = { type = 'imap' } },
    { when = { service = 'http' },             use = { type = 'http_inspect' } },
    { when = { service = 'http2' },            use = { type = 'http2_inspect' } },
    { when = { service = 'iec104' },           use = { type = 'iec104' } },
    { when = { service = 'mms' },              use = { type = 'mms' } },
    { when = { service = 'modbus' },           use = { type = 'modbus' } },
    { when = { service = 'pop3' },             use = { type = 'pop' } },
    { when = { service = 'ssh' },              use = { type = 'ssh' } },
    { when = { service = 'sip' },              use = { type = 'sip' } },
    { when = { service = 'smtp' },             use = { type = 'smtp' } },
    { when = { service = 'ssl' },              use = { type = 'ssl' } },
    { when = { service = 'sunrpc' },           use = { type = 'rpc_decode' } },
    { when = { service = 's7commplus' },       use = { type = 's7commplus' } },
    { when = { service = 'telnet' },           use = { type = 'telnet' } },

    { when = { proto = 'ip', nets = '[192.168.10.71, fe80::200:ff:fe02:71]'}, use = { type = 'stream_ip', name = 'stream_ip_7'} },
    { when = { proto = 'ip', nets = '[192.168.10.70, fe80::200:ff:fe02:70]'}, use = { type = 'stream_ip', name = 'stream_ip_6'} },
    { when = { proto = 'ip', nets = '[192.168.10.69, fe80::200:ff:fe02:69]'}, use = { type = 'stream_ip', name = 'stream_ip_5', }, },
    { when = { proto = 'ip', nets = '[192.168.10.68, fe80::200:ff:fe02:68]'}, use = { type = 'stream_ip', name = 'stream_ip_4', }, },
    { when = { proto = 'ip', nets = '[192.168.10.67, fe80::200:ff:fe02:67]'}, use = { type = 'stream_ip', name = 'stream_ip_3', }, },
    { when = { proto = 'ip', nets = '[192.168.10.66, fe80::200:ff:fe02:66]'}, use = { type = 'stream_ip', name = 'stream_ip_2', }, },
    { when = { proto = 'ip', nets = '[192.168.10.65, fe80::200:ff:fe02:65]'}, use = { type = 'stream_ip', name = 'stream_ip_1', }, },
    { when = { proto = 'ip', nets = '[192.168.10.64, fe80::200:ff:fe02:64]'}, use = { type = 'stream_ip', name = 'stream_ip_0', }, },





    { use = { type = 'wizard' } }
}

---------------------------------------------------------------------------
-- 4. configure performance
---------------------------------------------------------------------------

-- use latency to monitor / enforce packet and rule thresholds
--latency = { }

-- use these to capture perf data for analysis and tuning
--profiler = { }
--perf_monitor = { }

---------------------------------------------------------------------------
-- 5. configure detection
---------------------------------------------------------------------------

references = default_references
classifications = default_classifications

ips =
{
    -- use this to enable decoder and inspector alerts
    --enable_builtin_rules = true,

    -- use include for rules files; be sure to set your path
    -- note that rules files can include other rules files
    -- (see also related path vars at the top of snort_defaults.lua)

    variables = default_variables
}

-- use these to configure additional rule actions
-- react = { }
-- reject = { }

-- use this to enable payload injection utility
-- payload_injector = { }

---------------------------------------------------------------------------
-- 6. configure filters
---------------------------------------------------------------------------

-- below are examples of filters
-- each table is a list of records

--[[
suppress =
{
    -- don't want to any of see these
    { gid = 1, sid = 1 },

    -- don't want to see anything for a given host
    { track = 'by_dst', ip = '1.2.3.4' }

    -- don't want to see these for a given host
    { gid = 1, sid = 2, track = 'by_dst', ip = '1.2.3.4' },
}
--]]

--[[
event_filter =
{
    -- reduce the number of events logged for some rules
    --{ gid = 1, sid = 1, type = 'limit', track = 'by_src', count = 2, seconds = 10 },
    --{ gid = 1, sid = 2, type = 'both',  track = 'by_dst', count = 5, seconds = 60 },
    { gid = 1, count = 1000, seconds = 1 },
}
--]]

--[[
rate_filter =
{
    -- alert on connection attempts from clients in SOME_NET
    { gid = 135, sid = 1, track = 'by_src', count = 5, seconds = 1,
      new_action = 'alert', timeout = 4, apply_to = '[$SOME_NET]' },

    -- alert on connections to servers over threshold
    { gid = 135, sid = 2, track = 'by_dst', count = 29, seconds = 3,
      new_action = 'alert', timeout = 1 },
}
--]]

---------------------------------------------------------------------------
-- 7. configure outputs
---------------------------------------------------------------------------

-- event logging
-- you can enable with defaults from the command line with -A <alert_type>
-- uncomment below to set non-default configs
alert_csv = { file = true }
alert_fast = { file = true, packet = true}
alert_full = { file = true }
alert_sfsocket = { file = true }
--alert_syslog = { file = true }
--unified2 = { file = true }
alert_json =
{
file = true,
fields = 'timestamp class msg b64_data pkt_gen client_pkts client_bytes sgt',
}

-- packet logging
-- you can enable with defaults from the command line with -L <log_type>
--log_codecs = { }
--log_hext = { file = true  }
--log_pcap = { }

-- additional logs
packet_capture = { }
file_log = {  }

trace =
    {
        modules =
        {
            all = 5,        }
    }

---------------------------------------------------------------------------
-- 8. configure tweaks
---------------------------------------------------------------------------

if ( tweaks ~= nil ) then
    include(tweaks .. '.lua')
end

daq = pcap
