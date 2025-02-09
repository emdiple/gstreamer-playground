# Terminology
* **GStreamer** : 
A multimedia framework for processing media flows.
* **Source Elements (Producers)**:
The components that generate or provide the media data into the system.
* **Sink Elements (Consumers)**:
The components that receive or output the media data at the end of the processing chain.
* **Intermediate Elements (Fileters)**:
components that process, modify, or transform media as it flows through the pipeline.
* **Pipeline**:
The entire set of interconnected elements (sources, intermediates, and sinks) that together manage the flow and processing of multimedia data.
* **Playbin**:
A special element which acts as a source and as a sink, and is a whole pipeline[^1].
[^1]: Internally, it creates and connects all the necessary elements to play your media
* **Writable Properties**:
Named attributes of GStreamer elements that you can modify to change the element’s behavior.
* **Readable Properties**:
Named attributes that can be queried to determine an element’s current internal state.

