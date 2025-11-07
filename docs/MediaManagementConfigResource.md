# MediaManagementConfigResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**auto_unmonitor_previously_downloaded_tracks** | Option<**bool**> |  | [optional]
**recycle_bin** | Option<**String**> |  | [optional]
**recycle_bin_cleanup_days** | Option<**i32**> |  | [optional]
**download_propers_and_repacks** | Option<[**models::ProperDownloadTypes**](ProperDownloadTypes.md)> |  | [optional]
**create_empty_artist_folders** | Option<**bool**> |  | [optional]
**delete_empty_folders** | Option<**bool**> |  | [optional]
**file_date** | Option<[**models::FileDateType**](FileDateType.md)> |  | [optional]
**watch_library_for_changes** | Option<**bool**> |  | [optional]
**rescan_after_refresh** | Option<[**models::RescanAfterRefreshType**](RescanAfterRefreshType.md)> |  | [optional]
**allow_fingerprinting** | Option<[**models::AllowFingerprinting**](AllowFingerprinting.md)> |  | [optional]
**set_permissions_linux** | Option<**bool**> |  | [optional]
**chmod_folder** | Option<**String**> |  | [optional]
**chown_group** | Option<**String**> |  | [optional]
**skip_free_space_check_when_importing** | Option<**bool**> |  | [optional]
**minimum_free_space_when_importing** | Option<**i32**> |  | [optional]
**copy_using_hardlinks** | Option<**bool**> |  | [optional]
**enable_media_info** | Option<**bool**> |  | [optional]
**use_script_import** | Option<**bool**> |  | [optional]
**script_import_path** | Option<**String**> |  | [optional]
**import_extra_files** | Option<**bool**> |  | [optional]
**extra_file_extensions** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


