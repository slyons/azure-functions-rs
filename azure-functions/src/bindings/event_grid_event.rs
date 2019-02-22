use crate::rpc::protocol;
use chrono::{DateTime, Utc};
use serde_json::from_str;
use std::collections::HashMap;

/// Represents an Event Grid trigger binding.
///
/// # Examples
///
/// ```rust
/// # extern crate azure_functions;
/// # #[macro_use] extern crate log;
/// use azure_functions::{
///     bindings::EventGridEvent,
///     func,
/// };
///
/// #[func]
/// pub fn log_event(event: &EventGridEvent) {
///     log::warn!("Event Data: {}", event.data);
/// }
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventGridEvent {
    /// Full resource path to the event source.
    pub topic: String,
    /// Publisher-defined path to the event subject.
    pub subject: String,
    /// One of the registered event types for this event source.
    pub event_type: String,
    /// The time the event is generated based on the provider's UTC time.
    pub event_time: DateTime<Utc>,
    /// Unique identifier for the event.
    pub id: String,
    /// Event data specific to the resource provider.
    pub data: serde_json::Value,
    /// The schema version of the data object.
    pub data_version: String,
    /// The schema version of the event metadata.
    pub metadata_version: String,
}

impl EventGridEvent {
    #[doc(hidden)]
    pub fn new(data: protocol::TypedData, _: &mut HashMap<String, protocol::TypedData>) -> Self {
        if !data.has_json() {
            panic!("expected JSON data for Event Grid trigger binding");
        }

        from_str(data.get_json()).expect("failed to parse Event Grid JSON")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        const EVENT: &'static str = r#"{"topic":"/subscriptions/{subscription-id}/resourceGroups/Storage/providers/Microsoft.Storage/storageAccounts/xstoretestaccount","subject":"/blobServices/default/containers/oc2d2817345i200097container/blobs/oc2d2817345i20002296blob","eventType":"Microsoft.Storage.BlobCreated","eventTime":"2017-06-26T18:41:00.9584103Z","id":"831e1650-001e-001b-66ab-eeb76e069631","data":{"api":"PutBlockList","clientRequestId":"6d79dbfb-0e37-4fc4-981f-442c9ca65760","requestId":"831e1650-001e-001b-66ab-eeb76e000000","eTag":"0x8D4BCC2E4835CD0","contentType":"application/octet-stream","contentLength":524288,"blobType":"BlockBlob","url":"https://oc2d2817345i60006.blob.core.windows.net/oc2d2817345i200097container/oc2d2817345i20002296blob","sequencer":"00000000000004420000000000028963","storageDiagnostics":{"batchId":"b68529f3-68cd-4744-baa4-3c0498ec19f0"}},"dataVersion":"1","metadataVersion":"1"}"#;

        let mut data = protocol::TypedData::new();
        data.set_json(EVENT.to_string());

        let mut metadata = HashMap::new();

        let event = EventGridEvent::new(data, &mut metadata);
        assert_eq!(event.topic, "/subscriptions/{subscription-id}/resourceGroups/Storage/providers/Microsoft.Storage/storageAccounts/xstoretestaccount");
        assert_eq!(event.subject, "/blobServices/default/containers/oc2d2817345i200097container/blobs/oc2d2817345i20002296blob");
        assert_eq!(event.event_type, "Microsoft.Storage.BlobCreated");
        assert_eq!(
            event.event_time.to_rfc3339(),
            "2017-06-26T18:41:00.958410300+00:00"
        );
        assert_eq!(event.id, "831e1650-001e-001b-66ab-eeb76e069631");
        assert_eq!(event.data.to_string(), r#"{"api":"PutBlockList","blobType":"BlockBlob","clientRequestId":"6d79dbfb-0e37-4fc4-981f-442c9ca65760","contentLength":524288,"contentType":"application/octet-stream","eTag":"0x8D4BCC2E4835CD0","requestId":"831e1650-001e-001b-66ab-eeb76e000000","sequencer":"00000000000004420000000000028963","storageDiagnostics":{"batchId":"b68529f3-68cd-4744-baa4-3c0498ec19f0"},"url":"https://oc2d2817345i60006.blob.core.windows.net/oc2d2817345i200097container/oc2d2817345i20002296blob"}"#);
        assert_eq!(event.data_version, "1");
        assert_eq!(event.metadata_version, "1");
    }
}
