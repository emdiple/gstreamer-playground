# Comand Line tools for GStreamer
```
$ gst-inspect-1.0 [OPTION...] [PLUGIN|ELEMENT]
```
  * **Description**: gst-inspect-1.0 is a tool that prints out information on available GStreamer plugins, information about a particular plugin, or information about a particular element. When executed with no PLUGIN or ELEMENT argument, gst-inspect-1.0 will print a list of all plugins and elements together with a sumary. When executed with a PLUGIN or ELEMENT argument, gst-inspect-1.0 will print information about that plug-in or element.
  * **Options**: gst-inspect-1.0 accepts the following arguments and options:
    - `PLUGIN`<br />Name of a plugin. This is a file name like %GSTREAMER_ROOT_X86%\lib\gstreamer-1.0\libgstaudiotestsrc.dll for example.
    - `ELEMENT`<br />Name of an element. This is the name of an element, like audiotestsrc for example\
    - `--help`<br />Print help synopsis and available FLAGS
    -  `--gst-info-mask=FLAGS`<br />GStreamer info flags to set (list with --help)
    -  `-a, --print-all`<br />Print all plugins and elements
    -  `--print-plugin-auto-install-info`<br />Print a machine-parsable list of features the specified plugin provides. Useful in connection with external automatic plugin installation mechanisms.
    -  `--gst-debug-mask=FLAGS`<br />GStreamer debugging flags to set (list with --help)
    -  `--gst-mask=FLAGS`<br />GStreamer info and debugging flags to set (list with --help)
    -  `--gst-plugin-path=PATH`<br />Add directories separated with ':' to the plugin search path
    <br />
    <hr style="height:1px;border:none;color:transparent;background-color:#000;" />
```
$ gst-launch-1.0 [OPTIONS] PIPELINE-DESCRIPTION
```
* **Description**: gst-launch-1.0 is a tool that builds and runs basic GStreamer pipelines. In its simplest form, a PIPELINE-DESCRIPTION is a list of elements separated by exclamation marks (!). Properties may be appended to elements in the form property=value. A "preset" can also be set using the @preset=<preset name> syntax.
* **Pipeline Description**<br />
A pipeline consists of elements and links. Elements can be put into bins of different sorts. Elements, links, and bins can be specified in a pipeline description in any order.

* **Elements**<br />
ELEMENTTYPE [PROPERTY1 ...]
Creates an element of type ELEMENTTYPE and sets its PROPERTIES.

* **Element Properties**<br />
PROPERTY=VALUE ...
Sets the property to the specified value. You can use gst-inspect-1.0 to find out about properties and allowed values of different elements. Enumeration properties can be set by name, nick or value.

* **Element Presets**<br />
@preset=<preset name> ...
Sets the preset (basically a pre-made collection of property settings for a specific element) on the element. you can use gst-inspect-1.0 to find out what presets are available for a specific element.

* **Bins**<br />
[BINTYPE.] ([PROPERTY1 ...] PIPELINE-DESCRIPTION)
Specifies that a bin of type BINTYPE is created and the given properties are set. Every element between the braces is put into the bin. Please note the dot that has to be used after the BINTYPE. You will almost never need this functionality, it is only really useful for applications using the gst_parse_launch() API with bin as bintype. That way it is possible to build partial pipelines instead of a full-fledged top-level pipeline.

* **Links**<br />
```
[[SRCELEMENT].[PAD1,...]] ! [[SINKELEMENT].[PAD1,...]]
[[SRCELEMENT].[PAD1,...]] ! CAPS ! [[SINKELEMENT].[PAD1,...]]
[[SRCELEMENT].[PAD1,...]] : [[SINKELEMENT].[PAD1,...]]
[[SRCELEMENT].[PAD1,...]] : CAPS : [[SINKELEMENT].[PAD1,...]]
```
Links the element with name SRCELEMENT to the element with name SINKELEMENT, using the caps specified in CAPS as a filter.

Names can be set on elements using the name property. If the name is omitted, the element that was specified directly in front of or after the link is used. This works across bins. If a padname is given, the link is done using that pad. If no pad names are given all possibilities are tried and a compatible pad is used. If multiple padnames are given, both sides must have the same number of pads specified and multiple links are done in the given order.

The simplest link is a simple exclamation mark. This links the element to the left of it with the element at its right.

Linking using the : operator attempts to link all possible pads between the elements

The following links the element with name SRCELEMENT to the element with name SINKELEMENT, using the caps specified in CAPS as a filter:

[[SRCELEMENT].[PAD1,...]] ! CAPS ! [[SINKELEMENT].[PAD1,...]]
* **Caps**<br />
```
MIMETYPE [, PROPERTY[, PROPERTY ...]]] [; CAPS[; CAPS ...]]
```

Creates a capability with the given mimetype and optionally with given properties. The mimetype can be escaped using " or '. If you want to chain caps, you can add more caps in the same format afterwards.

* **Caps Properties**<br />
NAME=[(TYPE)] VALUE in lists and ranges: [(TYPE)] VALUE
Sets the requested property in capabilities. The name is an alphanumeric value and the type can have the following case-insensitive values:

i or int for integer values or ranges;
f or float for float values or ranges;
b, bool, or boolean for boolean values;
s, str, or string for strings;
fraction for fractions (framerate, pixel-aspect-ratio);
l or list for lists.
If no type was given, the following order is tried: integer, float, boolean, string. Integer values must be parsable by strtol(), floats by strtod(). Boolean values are (case insensitive) yes, no, true or false and may like strings be escaped with " or '.

Ranges are in this format: [VALUE, VALUE], e.g. width=[16,1920]

Lists use this format: {VALUE [, VALUE ...]}, e.g. width={1920,1280,640}

* **Pipeline Examples**<br />
The examples below assume that you have the correct plug-ins available. In general, "pulsesink" can be substituted with another audio output plug-in such as "alsasink", "osxaudiosink", or "wasapisink"

Likewise, xvimagesink can be substituted with d3dvideosink, ximagesink, sdlvideosink, osxvideosink, or aasink.

Keep in mind though that different sinks might accept different formats and even the same sink might accept different formats on different machines, so you might need to add converter elements like audioconvert and audioresample for audio or videoconvertscale in front of the sink to make things work.

  * **Audio playback**<br />
    Note: For audio/video playback it's best to use the playbin3 or uridecodebin3 elements, these are just example pipelines.

Play the mp3 music file "music.mp3" using a libmpg123-based plug-in and output it to an audio device via PulseAudio (or PipeWire).

gst-launch-1.0 filesrc location=music.mp3 ! mpegaudioparse ! mpg123audiodec ! audioconvert ! audioresample ! pulsesink
Play an Ogg Vorbis format file:

gst-launch-1.0 filesrc location=music.ogg ! oggdemux ! vorbisdec ! audioconvert ! audioresample ! pulsesink
Play an mp3 file or an http stream using GIO:

gst-launch-1.0 giosrc location=music.mp3 ! mpegaudioparse ! mpg123audiodec ! audioconvert ! pulsesink
gst-launch-1.0 giosrc location=http://domain.com/music.mp3 ! mpegaudioparse ! mpg123audiodec ! audioconvert ! audioresample ! pulsesink
Use GIO to play an mp3 file located on an SMB server:

gst-launch-1.0 giosrc location=smb://computer/music.mp3 ! mpegaudioparse ! mpg123audiodec ! audioconvert ! audioresample ! pulsesink
* **Format conversion**<br />
Convert an mp3 music file to an Ogg Vorbis file:

gst-launch-1.0 filesrc location=music.mp3 ! mpegaudioparse ! mpg123audiodec ! audioconvert ! vorbisenc ! oggmux ! filesink location=music.ogg
Convert to the FLAC format:

gst-launch-1.0 filesrc location=music.mp3 ! mpegaudioparse ! mpg123audiodec ! audioconvert ! flacenc ! filesink location=test.flac
* **Other**<br />
Play a .WAV file that contains raw audio data (PCM):

gst-launch-1.0 filesrc location=music.wav ! wavparse ! audioconvert ! audioresample ! pulsesink
Convert a .WAV file containing raw audio data into an Ogg Vorbis or mp3 file:

gst-launch-1.0 filesrc location=music.wav ! wavparse ! audioconvert ! vorbisenc ! oggmux ! filesink location=music.ogg
gst-launch-1.0 filesrc location=music.wav ! wavparse ! audioconvert ! lamemp3enc ! xingmux ! id3v2mux ! filesink location=music.mp3
Rip all tracks from CD and convert them into a single mp3 file:

gst-launch-1.0 cdparanoiasrc mode=continuous ! audioconvert ! lamemp3enc ! mpegaudioparse ! xingmux ! id3v2mux ! filesink location=cd.mp3
Rip track 5 from the CD and converts it into a single mp3 file:

gst-launch-1.0 cdparanoiasrc track=5 ! audioconvert ! lamemp3enc ! mpegaudioparse ! xingmux ! id3v2mux ! filesink location=track5.mp3
Using gst-inspect-1.0, it is possible to discover settings like the above for "cdparanoiasrc" that will tell it to rip the entire CD or only tracks of it. Alternatively, you can use an URI and gst-launch-1.0 will find an element (such as cdparanoia) that supports that protocol for you, e.g.:

gst-launch-1.0 cdda://5 ! lamemp3enc vbr=new vbr-quality=6 ! xingmux ! id3v2mux ! filesink location=track5.mp3
Record sound from your audio input and encode it into an ogg file:

gst-launch-1.0 pulsesrc ! audioconvert ! vorbisenc ! oggmux ! filesink location=input.ogg
* **Video**<br />
Note: For audio/video playback it's best to use the playbin3 or uridecodebin3 elements, these are just example pipelines.

Display only the video portion of an MPEG-2 video file, outputting to an X display window:

gst-launch-1.0 filesrc location=JB_FF9_TheGravityOfLove.mpg ! mpegdemux ! mpegvideoparse ! mpeg2dec ! videoconvert ! xvimagesink
Display the video portion of a .vob file (used on DVDs), outputting to an SDL window:

gst-launch-1.0 filesrc location=flflfj.vob ! dvddemux ! mpegvideoparse ! mpeg2dec ! videoconvert ! sdlvideosink
Play both video and audio portions of an MPEG movie:

gst-launch-1.0 filesrc location=movie.mpg ! dvddemux name=demuxer  \
\
demuxer. ! queue ! mpegvideoparse ! mpeg2dec ! videoconvert ! sdlvideosink \
demuxer. ! queue ! mpegaudioparse ! mpg123audiodec ! audioconvert ! audioresample ! pulsesink
Play an AVI movie with an external text subtitle stream:

This example shows how to refer to specific pads by name if an element (here: textoverlay) has multiple sink or source pads:

gst-launch-1.0 textoverlay name=overlay ! videoconvert ! videoscale ! autovideosink \
filesrc location=movie.avi ! decodebin3 !  videoconvert ! overlay.video_sink \
filesrc location=movie.srt ! subparse ! overlay.text_sink
Play an AVI movie with an external text subtitle stream using playbin:

gst-launch-1.0 playbin3 uri=<file:///path/to/movie.avi> suburi=<file:///path/to/movie.srt>
* **Network streaming**<br />
Stream video using RTP and network elements

This command would be run on the transmitter:

gst-launch-1.0 v4l2src ! queue ! videoconvert ! x264enc tune=zerolatency key-int-max=15 ! video/x-h264,profile=main ! rtph264pay pt=96 config-interval=-1 ! udpsink host=192.168.1.1 port=5000
Use this command on the receiver:

gst-launch-1.0 udpsrc port=5000 ! application/x-rtp,clock-rate=90000,payload=96 ! rtpjitterbuffer ! rtph264depay ! h264parse ! avdec_h264 ! videoconvert ! xvimagesink
* **Diagnostic**<br />
Generate a null stream and ignore it (and print out details):

gst-launch-1.0 -v fakesrc num-buffers=16 ! fakesink silent=false
Generate a pure sine tone to test the audio output:

gst-launch-1.0 audiotestsrc ! audioconvert ! audioresample ! osssink
Generate a familiar test pattern to test the video output:

gst-launch-1.0 videotestsrc ! ximagesink
gst-launch-1.0 videotestsrc ! xvimagesink
* **Automatic linking**<br />
You can use the "decodebin3" element to automatically select the right elements to get a working pipeline.

Play any supported audio format:

gst-launch-1.0 filesrc location=musicfile ! decodebin3 ! audioconvert ! audioresample ! pulsesink
Play any supported video format with video and audio output. Threads are used automatically:

gst-launch-1.0 filesrc location=videofile ! decodebin name=decoder decoder. ! queue ! audioconvert ! audioresample ! pulsesink   decoder. !  videoconvert ! xvimagesink
You can also support different inputs by using an URI and uridecodebin3, e.g.:

gst-launch-1.0 uridecodebin3 uri=file:///path/to/video.mp4 name=decoder decoder. ! queue ! audioconvert ! audioresample ! pulsesink   decoder. !  videoconvert ! xvimagesink
    gst-launch-1.0 uridecodebin3 uri=https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm name=decoder decoder. ! queue ! audioconvert ! audioresample ! pulsesink   decoder. !  videoconvert ! xvimagesink
To make the above even easier, you can use the playbin element:

    gst-launch-1.0 playbin3 uri=file:///home/joe/foo.avi 
<br />

    gst-launch-1.0 playbin3 uri=https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm
* **Filtered connections**<br />
These examples show you how to use filtered caps.

Show a test image and use the YUY2 or YV12 video format for this:

    gst-launch-1.0 videotestsrc ! 'video/x-raw,format=YUY2;video/x-raw,format=YV12' ! xvimagesink
or

    gst-launch-1.0 v4l2src ! image/jpeg ! queue ! decodebin3 ! videoconvert ! autovideosink
Record audio and write it to a .wav file. Force usage of signed 16 to 32 bit samples and a sample rate between 32kHz and 64KHz:

    gst-launch-1.0 pulsesrc !  'audio/x-raw,rate=[32000,64000],format={S16LE,S24LE,S32LE}' ! wavenc ! filesink location=recording.wav
* **Environment Variables**<br />
    `GST_DEBUG`: Comma-separated list of debug categories and levels, e.g:
        
        GST_DEBUG=totem:4,typefind:5
  * is allowed as a wildcard as part of debug category names (e.g. GST_DEBUG=*sink:6,*audio*:6). It is also possible to specify the log level by name (1=ERROR, 2=WARN, 3=FIXME, 4=INFO, 5=DEBUG, 6=LOG, 7=TRACE, 9=MEMDUMP), e.g. GST_DEBUG=*audio*:LOG.

`GST_DEBUG_NO_COLOR`: When this environment variable is set, coloured debug output is disabled. This might come handy when saving the debug output to a file.

`GST_DEBUG_DUMP_DOT_DIR`: When set to a filesystem path, store 'dot' files of pipeline graphs there.

These can then later be converted into an image using the 'dot' utility from the graphviz set of tools, like this: dot foo.dot -Tsvg -o foo.svg (png or jpg are also possible as output format). There is also a utility called xdot which allows you to view the .dot file directly without converting it first.

When the pipeline changes state through NULL to PLAYING and back to NULL, a dot file is generated on each state change. To write a snapshot of the pipeline state, send a SIGHUP to the process or use the pipeline_snapshot tracer from the GStreamer Rust plugins.

`GST_REGISTRY`: Path of the plugin registry file. The default is ~/.cache/gstreamer-1.0/registry-CPU.xml where CPU is the machine/cpu type GStreamer was compiled for, e.g. 'x86_64', etc. Check the output of uname -i and uname -m for details.

`GST_REGISTRY_UPDATE`: Set to "no" to force GStreamer to assume that no plugins have changed, have been added or have been removed. This will make GStreamer skip the initial check to determine whether a rebuild of the registry cache is required or not. This may be useful in embedded environments where the installed plugins never change. Do not use this option in any other setup.

`GST_PLUGIN_PATH`: Specifies a list of directories to scan for additional plugins. These take precedence over the system plugins.

`GST_PLUGIN_SYSTEM_PATH`: Specifies a list of plugins that are always loaded by default. If not set, this defaults to the system-installed path, and the plugins installed in the user's home directory

`GST_DEBUG_FILE`: Set this variable to a file path to redirect all GStreamer debug messages to this file. If left unset, debug messages will be output to the standard error output.

`ORC_CODE`: Useful Orc environment variable. Set ORC_CODE=debug to enable debuggers such as gdb to create useful backtraces from Orc-generated code. Set ORC_CODE=backup or ORC_CODE=emulate if you suspect Orc's SIMD code generator is producing incorrect code. (Quite a few important GStreamer plugins like videotestsrc, audioconvert or audioresample use Orc).

`G_DEBUG`: This is a useful GLib environment variable. Set G_DEBUG=fatal_warnings to make GStreamer programs abort when a critical warning such as an assertion failure occurs. This is useful if you want to find out which part of the code caused that warning to be triggered and under what circumstances. Simply set G_DEBUG as mentioned above and run the program under gdb (or let it core dump). Then get a stack trace in the usual way.









