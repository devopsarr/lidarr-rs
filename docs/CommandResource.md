# CommandResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**command_name** | Option<**String**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**body** | Option<[**models::Command**](Command.md)> |  | [optional]
**priority** | Option<[**models::CommandPriority**](CommandPriority.md)> |  | [optional]
**status** | Option<[**models::CommandStatus**](CommandStatus.md)> |  | [optional]
**result** | Option<[**models::CommandResult**](CommandResult.md)> |  | [optional]
**queued** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**started** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**ended** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**duration** | Option<**String**> |  | [optional]
**exception** | Option<**String**> |  | [optional]
**trigger** | Option<[**models::CommandTrigger**](CommandTrigger.md)> |  | [optional]
**client_user_agent** | Option<**String**> |  | [optional]
**state_change_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**send_updates_to_client** | Option<**bool**> |  | [optional]
**update_scheduled_task** | Option<**bool**> |  | [optional]
**last_execution_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


