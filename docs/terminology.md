* **Pipeline Description**:  
  A complete specification of a media processing workflow. A pipeline consists of elements, links, and optionally bins. These components can be specified in any order, and they work together to define how media data is processed.

* **Branch**:  
  A branch is a segment of a pipeline that processes a copy of the data stream independently. By branching, you can, for example, display a video stream on the screen while simultaneously recording it to a file.
The tee element facilitates this by having:​
  - One sink pad: Receives the input data.
  - Multiple source pads: Each outputs a copy of the input data to a different branch.

* **Elements**:  
  The basic building blocks in GStreamer that perform specific tasks (e.g., generating, processing, or consuming media data).  
  - **Syntax**:  
    ```
    ELEMENTTYPE [PROPERTY1 ...]
    ```  
    Creates an element of type `ELEMENTTYPE` and sets its properties.

* **Element Properties**:  
  Named attributes that control an element’s behavior or reveal its state.  
  - **Writable Properties**: Attributes that can be modified to change the element’s behavior.  
  - **Readable Properties**: Attributes that can be queried to determine the element’s internal state.  
  - **Syntax**:  
    ```
    PROPERTY=VALUE ...
    ```  
    *Tip*: Use `gst-inspect-1.0` to list properties and allowed values. Enumeration properties can be set by name, nickname, or value.

* **Element Presets**:  
  Pre-defined collections of property settings that can be applied to an element for common configurations.  
  - **Syntax**:  
    ```
    @preset=<preset name> ...
    ```  
    *Tip*: Use `gst-inspect-1.0` to view available presets for a specific element.

* **Bins**:  
  Containers used to group multiple elements within a pipeline, helping to manage complex setups.  
  - **Syntax**:  
    ```
    [BINTYPE.] ([PROPERTY1 ...] PIPELINE-DESCRIPTION)
    ```  
    Creates a bin of type `BINTYPE` with specified properties. All elements within the braces are included in the bin.  
  - *Note*: The dot after `BINTYPE` is required.

* **Links**:  
  Connections that define the data flow between elements in a pipeline.  
  - **Basic Link Syntax**:  
    ```
    [[SRCELEMENT].[PAD1,...]] ! [[SINKELEMENT].[PAD1,...]]
    ```  
    Links the element named `SRCELEMENT` to the element named `SINKELEMENT`.
  - **Link with Caps Filter**:  
    ```
    [[SRCELEMENT].[PAD1,...]] ! CAPS ! [[SINKELEMENT].[PAD1,...]]
    ```  
    Uses specified capabilities (`CAPS`) as a filter during linking.
  - **Alternate Operator (`:`)**:  
    Attempts to link all possible pads between the elements.
  - **Additional Notes**:  
    - Elements must be added to the same bin (or pipeline) before linking.  
    - Pad names can be specified for precise linking; if omitted, the element directly before or after the link is used.  
    - When multiple pad names are provided, both sides must have the same number, and links are established in the given order.

* **Caps (Capabilities)**:  
  Descriptors that specify the media type (mimetype) and associated properties (such as format, resolution, and framerate) for data flowing between elements.  
  - **Caps Syntax**:  
    ```
    MIMETYPE [, PROPERTY[, PROPERTY ...]] [; CAPS[; CAPS ...]]
    ```  
    Creates a caps structure that may include chained caps.
  - **Caps Properties**:  
    - **Syntax**:  
      ```
      NAME=[(TYPE)] VALUE
      ```  
      Sets the requested property in the capabilities.
    - **Allowed Types (case-insensitive)**:  
      - `i` or `int`: Integer values or ranges.
      - `f` or `float`: Floating-point values or ranges.
      - `b`, `bool`, or `boolean`: Boolean values.
      - `s`, `str`, or `string`: String values.
      - `fraction`: For fractions (e.g., framerate, pixel-aspect-ratio).
      - `l` or `list`: For lists.
    - **Ranges**: Specified as `[VALUE, VALUE]` (e.g., `width=[16,1920]`).
    - **Lists**: Specified as `{VALUE [, VALUE ...]}` (e.g., `width={1920,1280,640}`).

* **Muxers**:  
  Elements that combine multiple media streams (e.g., audio, video, subtitles) into a single container file.  
  - **Examples**:  
    - `qtmux`: Muxes streams into a QuickTime/MP4 container.  
    - `oggmux`: Muxes streams into an Ogg container.  
    - `matroskamux`: Muxes streams into a Matroska (MKV) file.  
  - **Purpose**:  
    To synchronize and package multiple streams into one playable file.

* **Demuxers**:  
  Elements that split a container file into its individual media streams.  
  - **Examples**:  
    - `mpegdemux`  
    - `dvddemux`  
  - **Purpose**:  
    To extract separate audio, video, and subtitle streams from a container.

* **Automatic Linking**:  
  Elements like `decodebin3`, `uridecodebin3`, and `playbin3` can automatically select and link the appropriate elements to build a working pipeline with minimal manual configuration.

* **Filtered Connections**:  
  Techniques that apply caps filters during linking to enforce specific media formats between elements.  
  - **Example**:  
    ```
    gst-launch-1.0 videotestsrc ! 'video/x-raw,format=YUY2;video/x-raw,format=YV12' ! xvimagesink
    ```

* **Bus**:  
  The messaging system in GStreamer that carries messages from the elements to the application.  
  - **Purpose**:  
    To deliver events such as errors, end-of-stream (EOS), warnings, and state changes from the pipeline.  
  - **Usage**:  
    - Retrieve the bus from a pipeline using functions like `gst_element_get_bus()`.  
    - Process messages either synchronously (e.g., using `gst_bus_timed_pop_filtered()`) or asynchronously with callbacks.

* **Pads**:  
    Pads are connection points on elements used for linking. Pad names can be specified to control which specific pads are connected. For the most part, all data in GStreamer flows one way through a link between elements. Data flows out of one element through one or more source pads, and elements accept incoming data through one or more sink pads. Source and sink elements have only source and sink pads, respectively. Through a process known as negotiation, two linked Pads agree on a common type, and thus the Capabilities of the Pads become fixed (they only have one type and do not contain ranges)



* **Pipeline Examples**:  
  Practical examples demonstrate various tasks:
  - **Audio Playback**:  
    Playing MP3, Ogg Vorbis, or WAV files using elements such as `mpegaudioparse`, `mpg123audiodec`, and `pulsesink`.
  - **Format Conversion**:  
    Converting media between formats (e.g., MP3 to Ogg Vorbis or FLAC).
  - **Video Playback**:  
    Displaying video files (e.g., MPEG, VOB, AVI) using elements like `mpegdemux`, `mpegvideoparse`, `mpeg2dec`, and `xvimagesink`.
  - **Network Streaming**:  
    Streaming video over RTP using elements such as `v4l2src`, `x264enc`, `rtph264pay`, and `udpsink` on the transmitter, with corresponding receiver elements.
  - **Diagnostics**:  
    Using `fakesrc` and `fakesink` to generate or ignore streams for testing purposes.

* **Environment Variables**:  
  Variables that influence GStreamer’s runtime behavior and debugging:
  - **GST_DEBUG**:  
    Comma-separated list of debug categories and levels (e.g., `GST_DEBUG=totem:4,typefind:5`).
  - **GST_DEBUG_NO_COLOR**:  
    Disables colored debug output.
  - **GST_DEBUG_DUMP_DOT_DIR**:  
    Specifies a directory to dump DOT files of pipeline graphs.
  - **GST_REGISTRY**:  
    Path to the plugin registry file (default: `~/.cache/gstreamer-1.0/registry-<CPU>.xml`).
  - **GST_REGISTRY_UPDATE**:  
    If set to `"no"`, GStreamer skips rebuilding the registry cache.
  - **GST_PLUGIN_PATH**:  
    Directories to search for additional plugins (these take precedence over system plugins).
  - **GST_PLUGIN_SYSTEM_PATH**:  
    Directories from which system plugins are loaded (defaults to the system-installed path).
  - **GST_DEBUG_FILE**:  
    File path to which debug messages are redirected.
  - **ORC_CODE**:  
    Controls Orc-generated code for SIMD optimizations (e.g., set to `debug` for detailed backtraces).
  - **G_DEBUG**:  
    A GLib variable (e.g., `G_DEBUG=fatal_warnings`) that can be set to cause GStreamer programs to abort on critical warnings.

* **Additional Concepts**:  
  - **Names**:  
    Elements can be given a custom name via the `name` property. If omitted, GStreamer assigns a unique name. Custom names help in retrieving elements later or for debugging.
  - **Playbin/Decodebin**:  
    High-level elements (`playbin3`, `uridecodebin3`) that automatically construct a pipeline to play media from a URI.
  - **Filtered Connections**:  
    Using caps to filter and enforce specific media formats during element linking.

<br />
<hr style=\"height:1px;border:none;color:transparent;background-color:#000;\" />
