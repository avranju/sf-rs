#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use std::string::FromUtf16Error;

use num_derive::{FromPrimitive, ToPrimitive};
use thiserror::Error as ThisError;
use windows::core::Error as WindowsError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Windows Error: {0}")]
    Windows(#[from] WindowsError),

    #[error("Utf16 Error: {0}")]
    Utf16StringDecode(#[from] FromUtf16Error),

    #[error("Call abandoned: {0}")]
    Abandoned(&'static str),

    #[error("Invalid service kind")]
    InvalidServiceKind,

    #[error("Invalid service partition kind")]
    InvalidServicePartitionKind,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum FabricErrorCode {
    /// Indicates that there was an unknown error.
    Unknown,

    /// Indicates the partition key is invalid.
    InvalidPartitionKey = NativeErrorCode::FABRIC_E_INVALID_PARTITION_KEY as u32,

    /// Indicates that certificate for user role () is not setup.
    UserRoleClientCertificateNotConfigured =
        NativeErrorCode::FABRIC_E_USER_ROLE_CLIENT_CERTIFICATE_NOT_CONFIGURED as u32,

    /// Indicates the Service Fabric Name already exists.
    NameAlreadyExists = NativeErrorCode::FABRIC_E_NAME_ALREADY_EXISTS as u32,

    /// Indicates the service already exists.
    ServiceAlreadyExists = NativeErrorCode::FABRIC_E_SERVICE_ALREADY_EXISTS as u32,

    /// Indicates the service group already exists.
    ServiceGroupAlreadyExists = NativeErrorCode::FABRIC_E_SERVICE_GROUP_ALREADY_EXISTS as u32,

    /// Indicates the application type is currently being provisioned.
    ApplicationTypeProvisionInProgress =
        NativeErrorCode::FABRIC_E_APPLICATION_TYPE_PROVISION_IN_PROGRESS as u32,

    /// Indicates the application type already exists.
    ApplicationTypeAlreadyExists = NativeErrorCode::FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS as u32,

    /// Indicates the application already exists.
    ApplicationAlreadyExists = NativeErrorCode::FABRIC_E_APPLICATION_ALREADY_EXISTS as u32,

    /// Indicates the Service Fabric Name was not found.
    NameNotFound = NativeErrorCode::FABRIC_E_NAME_DOES_NOT_EXIST as u32,

    /// Indicates the property was not found.
    PropertyNotFound = NativeErrorCode::FABRIC_E_PROPERTY_DOES_NOT_EXIST as u32,

    /// Indicates the value of the property was empty.
    PropertyValueEmpty = NativeErrorCode::FABRIC_E_VALUE_EMPTY as u32,

    /// Indicates the service was not found.
    ServiceNotFound = NativeErrorCode::FABRIC_E_SERVICE_DOES_NOT_EXIST as u32,

    /// Indicates the Auxiliary replica feature disabled.
    AuxiliaryFeatureDisabled = NativeErrorCode::FABRIC_E_AUXILIARY_FEATURE_DISABLED as u32,

    /// Indicates the service group was not found.
    ServiceGroupNotFound = NativeErrorCode::FABRIC_E_SERVICE_GROUP_DOES_NOT_EXIST as u32,

    /// Indicates the application type was not found.
    ApplicationTypeNotFound = NativeErrorCode::FABRIC_E_APPLICATION_TYPE_NOT_FOUND as u32,

    /// Indicates the application type is in use.
    ApplicationTypeInUse = NativeErrorCode::FABRIC_E_APPLICATION_TYPE_IN_USE as u32,

    /// Indicates the application does not exist.
    ApplicationNotFound = NativeErrorCode::FABRIC_E_APPLICATION_NOT_FOUND as u32,

    /// Indicates the service type was not found.
    ServiceTypeNotFound = NativeErrorCode::FABRIC_E_SERVICE_TYPE_NOT_FOUND as u32,

    /// Indicates that the service manifest was not found.
    ServiceManifestNotFound = NativeErrorCode::FABRIC_E_SERVICE_MANIFEST_NOT_FOUND as u32,

    /// Indicates the Service Fabric Name is not empty:
    /// there are entities such as child Names, Service or Properties associated with it.
    NameNotEmpty = NativeErrorCode::FABRIC_E_NAME_NOT_EMPTY as u32,

    /// Indicates the node was not found.
    NodeNotFound = NativeErrorCode::FABRIC_E_NODE_NOT_FOUND as u32,

    /// Indicates the node type or node type information was not found.
    NodeTypeNotFound = NativeErrorCode::FABRIC_E_NODE_TYPE_NOT_FOUND as u32,

    /// Indicates that the node is up when it is expected to be down.
    NodeIsUp = NativeErrorCode::FABRIC_E_NODE_IS_UP as u32,

    /// Indicates the replica is not the primary.
    NotPrimary = NativeErrorCode::FABRIC_E_NOT_PRIMARY as u32,

    /// Indicates the service partition does not have write quorum.
    NoWriteQuorum = NativeErrorCode::FABRIC_E_NO_WRITE_QUORUM as u32,

    /// Indicates that the reconfiguration is currently in pending state.
    ReconfigurationPending = NativeErrorCode::FABRIC_E_RECONFIGURATION_PENDING as u32,

    /// Indicates the replication queue is full.
    ReplicationQueueFull = NativeErrorCode::FABRIC_E_REPLICATION_QUEUE_FULL as u32,

    /// Indicates that the replication operation is too large.
    ReplicationOperationTooLarge = NativeErrorCode::FABRIC_E_REPLICATION_OPERATION_TOO_LARGE as u32,

    /// Indicates that the atomic group is invalid.
    InvalidAtomicGroup = NativeErrorCode::FABRIC_E_INVALID_ATOMIC_GROUP as u32,

    /// Indicates the service is offline.
    ServiceOffline = NativeErrorCode::FABRIC_E_SERVICE_OFFLINE as u32,

    /// Indicates the partition was not found.
    PartitionNotFound = NativeErrorCode::FABRIC_E_PARTITION_NOT_FOUND as u32,

    /// Two  objects cannot be compared using  
    /// because they describe different replica sets.
    ServiceMetadataMismatch = NativeErrorCode::FABRIC_E_SERVICE_METADATA_MISMATCH as u32,

    /// Indicates that the service affinity chain is not supported.
    ServiceAffinityChainNotSupported =
        NativeErrorCode::FABRIC_E_SERVICE_AFFINITY_CHAIN_NOT_SUPPORTED as u32,

    /// Indicates there was a write conflict.
    WriteConflict = NativeErrorCode::FABRIC_E_WRITE_CONFLICT as u32,

    /// Indicates the application upgrade validation failed.
    ApplicationUpgradeValidationError =
        NativeErrorCode::FABRIC_E_APPLICATION_UPGRADE_VALIDATION_ERROR as u32,

    /// Indicates that there is a mismatch in the service type.
    ServiceTypeMismatch = NativeErrorCode::FABRIC_E_SERVICE_TYPE_MISMATCH as u32,

    /// Indicates the service template was not found.
    ServiceTemplateNotFound = NativeErrorCode::FABRIC_E_SERVICE_TYPE_TEMPLATE_NOT_FOUND as u32,

    /// Indicates the service type was already registered.
    ServiceTypeAlreadyRegistered = NativeErrorCode::FABRIC_E_SERVICE_TYPE_ALREADY_REGISTERED as u32,

    /// Indicates the service type was not registered.
    ServiceTypeNotRegistered = NativeErrorCode::FABRIC_E_SERVICE_TYPE_NOT_REGISTERED as u32,

    /// Indicates the application is currently being upgraded.
    ApplicationUpgradeInProgress = NativeErrorCode::FABRIC_E_APPLICATION_UPGRADE_IN_PROGRESS as u32,

    /// A FabricErrorCode that indicates the application is currently being updated.
    ApplicationUpdateInProgress = NativeErrorCode::FABRIC_E_APPLICATION_UPDATE_IN_PROGRESS as u32,

    /// Indicates that the upgrade domain was already completed.
    UpgradeDomainAlreadyCompleted =
        NativeErrorCode::FABRIC_E_UPGRADE_DOMAIN_ALREADY_COMPLETED as u32,

    /// The specified code or Cluster Manifest version cannot be unprovisioned or used as the target of an upgrade because it has not been provisioned.
    FabricVersionNotFound = NativeErrorCode::FABRIC_E_FABRIC_VERSION_NOT_FOUND as u32,

    /// The specified code or Cluster Manifest version cannot be unprovisioned because it is either being used by the cluster or is the target of a cluster upgrade.
    FabricVersionInUse = NativeErrorCode::FABRIC_E_FABRIC_VERSION_IN_USE as u32,

    /// The specified code or Cluster Manifest version has already been provisioned in the system.
    FabricVersionAlreadyExists = NativeErrorCode::FABRIC_E_FABRIC_VERSION_ALREADY_EXISTS as u32,

    /// The Service Fabric cluster is already in the target code or Cluster Manifest version specified by the upgrade request.
    FabricAlreadyInTargetVersion =
        NativeErrorCode::FABRIC_E_FABRIC_ALREADY_IN_TARGET_VERSION as u32,

    /// The Service Fabric cluster is not currently being upgrade and the request is only valid during upgrade.
    FabricNotUpgrading = NativeErrorCode::FABRIC_E_FABRIC_NOT_UPGRADING as u32,

    /// The Service Fabric Cluster is currently begin upgraded and the request is not valid during upgrade.
    FabricUpgradeInProgress = NativeErrorCode::FABRIC_E_FABRIC_UPGRADE_IN_PROGRESS as u32,

    /// An error in the Service Fabric cluster upgrade request was discovered during pre-upgrade validation of the upgrade description and Cluster Manifest.
    FabricUpgradeValidationError = NativeErrorCode::FABRIC_E_FABRIC_UPGRADE_VALIDATION_ERROR as u32,

    /// Indicates that  the health client has reached the maximum number of health reports that can accept for processing. More reports will be accepted when progress is done with the currently accepted reports. By default, the health client can accept 10000 different reports.
    FabricHealthMaxReportsReached = NativeErrorCode::FABRIC_E_HEALTH_MAX_REPORTS_REACHED as u32,

    /// Indicates the report is stale. Returned when health client has a report for the same entity, health information property with same or higher health sequence number.
    FabricHealthStaleReport = NativeErrorCode::FABRIC_E_HEALTH_STALE_REPORT as u32,

    /// Indicates the health entity is not found in the Health Store. Returned when Health Store has no reports from a Service Fabric system component on the entity or on one of the hierarchical parents. This can happen if the entity or one of its parents doesn't exist in the Service Fabric cluster, or the reports didn't yet arrive at the health store.
    FabricHealthEntityNotFound = NativeErrorCode::FABRIC_E_HEALTH_ENTITY_NOT_FOUND as u32,

    /// Indicates Service Fabric service is too busy to accept requests at this time. This is a transient error.
    ServiceTooBusy = NativeErrorCode::FABRIC_E_SERVICE_TOO_BUSY as u32,

    /// Indicates a communication error has occurred.
    CommunicationError = NativeErrorCode::FABRIC_E_COMMUNICATION_ERROR as u32,

    /// Indicates that the gateway could not be reached. This is a transient error.
    GatewayNotReachable = NativeErrorCode::FABRIC_E_GATEWAY_NOT_REACHABLE as u32,

    /// Indicates the object was closed.
    ObjectClosed = NativeErrorCode::FABRIC_E_OBJECT_CLOSED as u32,

    /// Indicates the a Check  has failed.
    PropertyCheckFailed = NativeErrorCode::FABRIC_E_PROPERTY_CHECK_FAILED as u32,

    /// Indicates the enumeration completed.
    EnumerationCompleted = NativeErrorCode::FABRIC_E_ENUMERATION_COMPLETED as u32,

    /// Indicates the configuration section was not found.
    ConfigurationSectionNotFound = NativeErrorCode::FABRIC_E_CONFIGURATION_SECTION_NOT_FOUND as u32,

    /// Indicates the configuration parameter was not found.
    ConfigurationParameterNotFound =
        NativeErrorCode::FABRIC_E_CONFIGURATION_PARAMETER_NOT_FOUND as u32,

    /// Indicates the configuration was invalid.
    InvalidConfiguration = NativeErrorCode::FABRIC_E_INVALID_CONFIGURATION as u32,

    /// Indicates the image builder validation error as occurred.
    ImageBuilderValidationError = NativeErrorCode::FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR as u32,

    /// Indicates the path passed by the user starts with a reserved directory.
    ImageBuilderReservedDirectoryError =
        NativeErrorCode::FABRIC_E_IMAGEBUILDER_RESERVED_DIRECTORY_ERROR as u32,

    /// Indicates the replica does not exist.
    ReplicaDoesNotExist = NativeErrorCode::FABRIC_E_REPLICA_DOES_NOT_EXIST as u32,

    /// Indicates the process got deactivated.
    ProcessDeactivated = NativeErrorCode::FABRIC_E_PROCESS_DEACTIVATED as u32,

    /// Indicates the process aborted.
    ProcessAborted = NativeErrorCode::FABRIC_E_PROCESS_ABORTED as u32,

    /// Indicates the address was invalid.
    InvalidAddress = NativeErrorCode::FABRIC_E_INVALID_ADDRESS as u32,

    /// Indicates the URI is invalid.
    InvalidNameUri = NativeErrorCode::FABRIC_E_INVALID_NAME_URI as u32,

    /// Indicates the property value is too large.
    ValueTooLarge = NativeErrorCode::FABRIC_E_VALUE_TOO_LARGE as u32,

    /// Indicates the directory was not found.
    DirectoryNotFound = NativeErrorCode::FABRIC_E_DIRECTORY_NOT_FOUND as u32,

    /// Indicates the path is too long.
    PathTooLong = NativeErrorCode::FABRIC_E_PATH_TOO_LONG as u32,

    /// Indicates the file was not found.
    FileNotFound = NativeErrorCode::FABRIC_E_FILE_NOT_FOUND as u32,

    /// Indicates that the code is not ready.
    NotReady = NativeErrorCode::FABRIC_E_NOT_READY as u32,

    /// Indicates the operation timed out.
    OperationTimedOut = NativeErrorCode::FABRIC_E_TIMEOUT as u32,

    /// Indicates the operation did not completed.
    OperationNotComplete = NativeErrorCode::FABRIC_E_OPERATION_NOT_COMPLETE as u32,

    /// Indicates the code package was not found.
    CodePackageNotFound = NativeErrorCode::FABRIC_E_CODE_PACKAGE_NOT_FOUND as u32,

    /// A that configuration package was not found.
    ConfigurationPackageNotFound = NativeErrorCode::FABRIC_E_CONFIGURATION_PACKAGE_NOT_FOUND as u32,

    /// Indicates the data package was not found.
    DataPackageNotFound = NativeErrorCode::FABRIC_E_DATA_PACKAGE_NOT_FOUND as u32,

    /// Indicates the endpoint resource was not found.
    EndpointResourceNotFound = NativeErrorCode::FABRIC_E_SERVICE_ENDPOINT_RESOURCE_NOT_FOUND as u32,

    /// Indicates the credential type is invalid.
    InvalidCredentialType = NativeErrorCode::FABRIC_E_INVALID_CREDENTIAL_TYPE as u32,

    /// Indicates that the X509FindType is invalid.
    InvalidX509FindType = NativeErrorCode::FABRIC_E_INVALID_X509_FIND_TYPE as u32,

    /// Indicates that the X509 store location is invalid.
    InvalidX509StoreLocation = NativeErrorCode::FABRIC_E_INVALID_X509_STORE_LOCATION as u32,

    /// Indicates that the X509 store name is invalid.
    InvalidX509StoreName = NativeErrorCode::FABRIC_E_INVALID_X509_STORE_NAME as u32,

    /// Indicates that the X509 certificate thumbprint string is invalid.
    InvalidX509Thumbprint = NativeErrorCode::FABRIC_E_INVALID_X509_THUMBPRINT as u32,

    /// Indicates the protection level is invalid.
    InvalidProtectionLevel = NativeErrorCode::FABRIC_E_INVALID_PROTECTION_LEVEL as u32,

    /// Indicates that the X509 certificate store cannot be opened.
    InvalidX509Store = NativeErrorCode::FABRIC_E_INVALID_X509_STORE as u32,

    /// Indicates the subject name is invalid.
    InvalidSubjectName = NativeErrorCode::FABRIC_E_INVALID_SUBJECT_NAME as u32,

    /// Indicates that the format of common name list string is invalid. It should be a comma separated list
    InvalidAllowedCommonNameList =
        NativeErrorCode::FABRIC_E_INVALID_ALLOWED_COMMON_NAME_LIST as u32,

    /// Indicates the credentials are invalid.
    InvalidCredentials = NativeErrorCode::FABRIC_E_INVALID_CREDENTIALS as u32,

    /// Indicates the decryption failed.
    DecryptionFailed = NativeErrorCode::FABRIC_E_DECRYPTION_FAILED as u32,

    /// Indicates that the encryption has failed.
    EncryptionFailed = NativeErrorCode::FABRIC_E_ENCRYPTION_FAILED as u32,

    /// Indicates the image store object was corrupted.
    CorruptedImageStoreObjectFound =
        NativeErrorCode::FABRIC_E_CORRUPTED_IMAGE_STORE_OBJECT_FOUND as u32,

    /// Indicates that the ImageBuilder hit an unexpected error.
    ImageBuilderUnexpectedError = NativeErrorCode::FABRIC_E_IMAGEBUILDER_UNEXPECTED_ERROR as u32,

    /// Indicates that the ImageBuilder was not able to perform the operation in the specified timeout.
    ImageBuilderTimeoutError = NativeErrorCode::FABRIC_E_IMAGEBUILDER_TIMEOUT as u32,

    /// Indicates that the ImageBuilder hit an AccessDeniedException when using the ImageStore.
    ImageBuilderAccessDeniedError = NativeErrorCode::FABRIC_E_IMAGEBUILDER_ACCESS_DENIED as u32,

    /// Indicates that the MSI file being provisioned is not valid.
    ImageBuilderInvalidMsiFile = NativeErrorCode::FABRIC_E_IMAGEBUILDER_INVALID_MSI_FILE as u32,

    /// Indicates there was an ImageStoreIOEception.
    ImageStoreIOException = NativeErrorCode::FABRIC_E_IMAGESTORE_IOERROR as u32,

    /// A FabricErrorCode that indicates that the operation failed to acquire a lock.
    ImageStoreAcquireFileLockFailed = NativeErrorCode::FABRIC_E_ACQUIRE_FILE_LOCK_FAILED as u32,

    /// Indicates that the ServiceType was not defined in the service manifest.
    InvalidServiceType = NativeErrorCode::FABRIC_E_INVALID_SERVICE_TYPE as u32,

    /// The application is not currently being upgraded and the request is only valid during upgrade.
    ApplicationNotUpgrading = NativeErrorCode::FABRIC_E_APPLICATION_NOT_UPGRADING as u32,

    /// The application is already in the target version specified by an application upgrade request.
    ApplicationAlreadyInTargetVersion =
        NativeErrorCode::FABRIC_E_APPLICATION_ALREADY_IN_TARGET_VERSION as u32,

    /// Indicates that the key is too large.
    KeyTooLarge = NativeErrorCode::FABRIC_E_KEY_TOO_LARGE as u32,

    /// Indicates that the key cannot be found.
    KeyNotFound = NativeErrorCode::FABRIC_E_KEY_NOT_FOUND as u32,

    /// Indicates the sequence number check failed. This usually happens when there is a conflicting operation executed on the same object which modifies the sequence number.
    SequenceNumberCheckFailed = NativeErrorCode::FABRIC_E_SEQUENCE_NUMBER_CHECK_FAILED as u32,

    /// Indicates that the transaction is not active because it has already been committed or rolled back.
    TransactionNotActive = NativeErrorCode::FABRIC_E_TRANSACTION_NOT_ACTIVE as u32,

    /// Indicates that the transaction is too large, which typically results when the transaction either contains too many operations or the size of the data being written is too large.
    TransactionTooLarge = NativeErrorCode::FABRIC_E_TRANSACTION_TOO_LARGE as u32,

    /// FabricErrorCode that indicates that one transaction can't be used by multiple threads simultaneously.
    MultithreadedTransactionsNotAllowed =
        NativeErrorCode::FABRIC_E_MULTITHREADED_TRANSACTIONS_NOT_ALLOWED as u32,

    /// FabricErrorCode that indicates that the transaction was aborted.
    TransactionAborted = NativeErrorCode::FABRIC_E_TRANSACTION_ABORTED as u32,

    /// Indicates that the reliable session transport startup has failed.
    ReliableSessionTransportStartupFailure =
        NativeErrorCode::FABRIC_E_RELIABLE_SESSION_TRANSPORT_STARTUP_FAILURE as u32,

    /// Indicates that the reliable session already exists.
    ReliableSessionAlreadyExists = NativeErrorCode::FABRIC_E_RELIABLE_SESSION_ALREADY_EXISTS as u32,

    /// Indicates that the reliable session cannot connect.
    ReliableSessionCannotConnect = NativeErrorCode::FABRIC_E_RELIABLE_SESSION_CANNOT_CONNECT as u32,

    /// Indicates that the reliable session manager already exists.
    ReliableSessionManagerExists = NativeErrorCode::FABRIC_E_RELIABLE_SESSION_MANAGER_EXISTS as u32,

    /// Indicates that the reliable session was rejected.
    ReliableSessionRejected = NativeErrorCode::FABRIC_E_RELIABLE_SESSION_REJECTED as u32,

    /// Indicates that the reliable session cannot be found.
    ReliableSessionNotFound = NativeErrorCode::FABRIC_E_RELIABLE_SESSION_NOT_FOUND as u32,

    /// Indicates that the reliable session queue is empty.
    ReliableSessionQueueEmpty = NativeErrorCode::FABRIC_E_RELIABLE_SESSION_QUEUE_EMPTY as u32,

    /// Indicates that the reliable session quota exceeded.
    ReliableSessionQuotaExceeded = NativeErrorCode::FABRIC_E_RELIABLE_SESSION_QUOTA_EXCEEDED as u32,

    /// Indicates that the reliable session service was faulted.
    ReliableSessionServiceFaulted =
        NativeErrorCode::FABRIC_E_RELIABLE_SESSION_SERVICE_FAULTED as u32,

    /// Indicates that the reliable session manager is already listening.
    ReliableSessionManagerAlreadyListening =
        NativeErrorCode::FABRIC_E_RELIABLE_SESSION_MANAGER_ALREADY_LISTENING as u32,

    /// Indicates that the reliable session manager was not found.
    ReliableSessionManagerNotFound =
        NativeErrorCode::FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_FOUND as u32,

    /// Indicates that the reliable session manager is not listening.
    ReliableSessionManagerNotListening =
        NativeErrorCode::FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_LISTENING as u32,

    /// Indicates that the reliable session has invalid target partition.
    ReliableSessionInvalidTargetPartition =
        NativeErrorCode::FABRIC_E_RELIABLE_SESSION_INVALID_TARGET_PARTITION as u32,

    /// Indicates that the repair task already exists.
    RepairTaskAlreadyExists = NativeErrorCode::FABRIC_E_REPAIR_TASK_ALREADY_EXISTS as u32,

    /// Indicates that the repair task was not found.
    RepairTaskNotFound = NativeErrorCode::FABRIC_E_REPAIR_TASK_NOT_FOUND as u32,

    /// Indicates that the instance identifier doesn�t match.
    InstanceIdMismatch = NativeErrorCode::FABRIC_E_INSTANCE_ID_MISMATCH as u32,

    /// Indicates that the node has not stopped yet.
    NodeHasNotStoppedYet = NativeErrorCode::FABRIC_E_NODE_HAS_NOT_STOPPED_YET as u32,

    /// Indicates that the cluster capacity is insufficient.
    InsufficientClusterCapacity = NativeErrorCode::FABRIC_E_INSUFFICIENT_CLUSTER_CAPACITY as u32,

    /// A FabricErrorCode that indicates the specified constraint key is undefined.
    ConstraintKeyUndefined = NativeErrorCode::FABRIC_E_CONSTRAINT_KEY_UNDEFINED as u32,

    /// A FabricErrorCode that indicates the package sharing policy is incorrect.
    InvalidPackageSharingPolicy = NativeErrorCode::FABRIC_E_INVALID_PACKAGE_SHARING_POLICY as u32,

    /// Predeployed of application package on Node not allowed. Predeployment feature requires ImageCache to be enabled on node.
    PreDeploymentNotAllowed = NativeErrorCode::FABRIC_E_PREDEPLOYMENT_NOT_ALLOWED as u32,

    /// Invalid backup setting. E.g. incremental backup option is not set upfront etc.
    InvalidBackupSetting = NativeErrorCode::FABRIC_E_INVALID_BACKUP_SETTING as u32,

    /// Cannot restart a replica of a volatile stateful service or an instance of a stateless service
    InvalidReplicaOperation = NativeErrorCode::FABRIC_E_INVALID_REPLICA_OPERATION as u32,

    /// The replica is currently transitioning (closing or opening) and the operation cannot be performed at this time
    InvalidReplicaStateForReplicaOperation = NativeErrorCode::FABRIC_E_INVALID_REPLICA_STATE as u32,

    /// Incremental backups can only be done after an initial full backup.
    MissingFullBackup = NativeErrorCode::FABRIC_E_MISSING_FULL_BACKUP as u32,

    /// A backup is currently in progress.
    BackupInProgress = NativeErrorCode::FABRIC_E_BACKUP_IN_PROGRESS as u32,

    /// The Cluster Resource Balancer is not yet ready to handle the operation.
    PLBNotReady = NativeErrorCode::FABRIC_E_LOADBALANCER_NOT_READY as u32,

    /// Indicates that a service notification filter has already been registered at the specified name by the current Fabric Client.
    DuplicateServiceNotificationFilterName =
        NativeErrorCode::FABRIC_E_DUPLICATE_SERVICE_NOTIFICATION_FILTER_NAME as u32,

    /// A FabricErrorCode that indicates the partition operation is invalid.
    InvalidPartitionOperation = NativeErrorCode::FABRIC_E_INVALID_PARTITION_OPERATION as u32,

    /// The replica already has Primary role.
    AlreadyPrimaryReplica = NativeErrorCode::FABRIC_E_PRIMARY_ALREADY_EXISTS as u32,

    /// The replica already has Secondary role.
    AlreadySecondaryReplica = NativeErrorCode::FABRIC_E_SECONDARY_ALREADY_EXISTS as u32,

    /// The stateless instance already exists
    AlreadyInstance = NativeErrorCode::FABRIC_E_INSTANCE_ALREADY_EXISTS as u32,

    /// The replica already has Auxiliary role.
    AlreadyAuxiliaryReplica = NativeErrorCode::FABRIC_E_AUXILIARY_ALREADY_EXISTS as u32,

    /// The backup directory is not empty.
    BackupDirectoryNotEmpty = NativeErrorCode::FABRIC_E_BACKUP_DIRECTORY_NOT_EMPTY as u32,

    /// The replica belongs to a self-activated service. The ForceRemove option is not supported for such replicas
    ForceNotSupportedForReplicaControlOperation =
        NativeErrorCode::FABRIC_E_FORCE_NOT_SUPPORTED_FOR_REPLICA_OPERATION as u32,

    /// A FabricErrorCode that indicates the connection was denied by the remote side.
    ConnectionDenied = NativeErrorCode::FABRIC_E_CONNECTION_DENIED as u32,

    /// A FabricErrorCode that indicates the authentication failed.
    ServerAuthenticationFailed = NativeErrorCode::FABRIC_E_SERVER_AUTHENTICATION_FAILED as u32,

    /// A FabricErrorCode that indicates there was a connection failure.
    FabricCannotConnect = NativeErrorCode::FABRIC_E_CANNOT_CONNECT as u32,

    /// A FabricErrorCode that indicates the connection was closed by the remote end.
    FabricConnectionClosedByRemoteEnd =
        NativeErrorCode::FABRIC_E_CONNECTION_CLOSED_BY_REMOTE_END as u32,

    /// A FabricErrorCode that indicates the message is too large.
    FabricMessageTooLarge = NativeErrorCode::FABRIC_E_MESSAGE_TOO_LARGE as u32,

    /// The service's and cluster's configuration settings would result in a constraint-violating state if the operation were executed.
    ConstraintNotSatisfied = NativeErrorCode::FABRIC_E_CONSTRAINT_NOT_SATISFIED as u32,

    /// A FabricErrorCode that indicates the specified endpoint was not found.
    FabricEndpointNotFound = NativeErrorCode::FABRIC_E_ENDPOINT_NOT_FOUND as u32,

    /// A FabricErrorCode that indicates that an object appears more than once in an array of synchronization objects.
    DuplicateWaitObject = NativeErrorCode::COR_E_DUPLICATEWAITOBJECT as u32,

    /// A FabricErrorCode that indicates that the entry point was not found. This happens when type loading failures occur.
    EntryPointNotFound = NativeErrorCode::COR_E_TYPELOAD as u32,

    /// Deletion of backup files/directory failed. Currently this can happen
    /// in a scenario where backup is used mainly to truncate logs.
    DeleteBackupFileFailed = NativeErrorCode::FABRIC_E_DELETE_BACKUP_FILE_FAILED as u32,

    /// Operation was canceled by the system and should be retried by the client.
    OperationCanceled = NativeErrorCode::E_ABORT as u32,

    /// A FabricErrorCode that indicates that this API call is not valid for the current state of the test command.
    InvalidTestCommandState = NativeErrorCode::FABRIC_E_INVALID_TEST_COMMAND_STATE as u32,

    /// A FabricErrorCode that indicates that this test command operation id (Guid) is already being used.
    TestCommandOperationIdAlreadyExists =
        NativeErrorCode::FABRIC_E_TEST_COMMAND_OPERATION_ID_ALREADY_EXISTS as u32,

    /// A FabricErrorCode that indicates that an instance of Chaos is already running.
    ChaosAlreadyRunning = NativeErrorCode::FABRIC_E_CHAOS_ALREADY_RUNNING as u32,

    /// Creation or deletion terminated due to persistent failures after bounded retry.
    CMOperationFailed = NativeErrorCode::FABRIC_E_CM_OPERATION_FAILED as u32,

    /// Fabric Data Root is not defined on the target machine.
    FabricDataRootNotFound = NativeErrorCode::FABRIC_E_FABRIC_DATA_ROOT_NOT_FOUND as u32,

    /// Indicates that restore metadata present in supplied restore directory in invalid.
    InvalidRestoreData = NativeErrorCode::FABRIC_E_INVALID_RESTORE_DATA as u32,

    /// Indicates that backup-chain in specified restore directory contains duplicate backups.
    DuplicateBackups = NativeErrorCode::FABRIC_E_DUPLICATE_BACKUPS as u32,

    /// Indicates that backup-chain in specified restore directory has one or more missing backups.
    InvalidBackupChain = NativeErrorCode::FABRIC_E_INVALID_BACKUP_CHAIN as u32,

    /// An operation is already in progress.
    StopInProgress = NativeErrorCode::FABRIC_E_STOP_IN_PROGRESS as u32,

    /// The node is already in a stopped state
    AlreadyStopped = NativeErrorCode::FABRIC_E_ALREADY_STOPPED as u32,

    /// The node is down (not stopped).
    NodeIsDown = NativeErrorCode::FABRIC_E_NODE_IS_DOWN as u32,

    /// Node transition in progress
    NodeTransitionInProgress = NativeErrorCode::FABRIC_E_NODE_TRANSITION_IN_PROGRESS as u32,

    /// The provided instance id is invalid.
    InvalidInstanceId = NativeErrorCode::FABRIC_E_INVALID_INSTANCE_ID as u32,

    /// The provided duration is invalid.
    InvalidDuration = NativeErrorCode::FABRIC_E_INVALID_DURATION as u32,

    /// Indicates that backup provided for restore is invalid.
    InvalidBackup = NativeErrorCode::FABRIC_E_INVALID_BACKUP as u32,

    /// Indicates that backup provided for restore has older data than present in service.
    RestoreSafeCheckFailed = NativeErrorCode::FABRIC_E_RESTORE_SAFE_CHECK_FAILED as u32,

    /// Indicates that the config upgrade fails.
    ConfigUpgradeFailed = NativeErrorCode::FABRIC_E_CONFIG_UPGRADE_FAILED as u32,

    /// Indicates that the upload session range will overlap or are out of range.
    UploadSessionRangeNotSatisfiable =
        NativeErrorCode::FABRIC_E_UPLOAD_SESSION_RANGE_NOT_SATISFIABLE as u32,

    /// Indicates that the upload session ID is existed for a different image store relative path.
    UploadSessionIdConflict = NativeErrorCode::FABRIC_E_UPLOAD_SESSION_ID_CONFLICT as u32,

    /// Indicates that the partition selector is invalid.
    InvalidPartitionSelector = NativeErrorCode::FABRIC_E_INVALID_PARTITION_SELECTOR as u32,

    /// Indicates that the replica selector is invalid.
    InvalidReplicaSelector = NativeErrorCode::FABRIC_E_INVALID_REPLICA_SELECTOR as u32,

    /// Indicates that DnsService is not enabled on the cluster.
    DnsServiceNotFound = NativeErrorCode::FABRIC_E_DNS_SERVICE_NOT_FOUND as u32,

    /// Indicates that service DNS name is invalid.
    InvalidDnsName = NativeErrorCode::FABRIC_E_INVALID_DNS_NAME as u32,

    /// Indicates that service DNS name is in use by another service.
    DnsNameInUse = NativeErrorCode::FABRIC_E_DNS_NAME_IN_USE as u32,

    /// Indicates the compose deployment already exists.
    ComposeDeploymentAlreadyExists =
        NativeErrorCode::FABRIC_E_COMPOSE_DEPLOYMENT_ALREADY_EXISTS as u32,

    /// Indicates the compose deployment is not found.
    ComposeDeploymentNotFound = NativeErrorCode::FABRIC_E_COMPOSE_DEPLOYMENT_NOT_FOUND as u32,

    /// Indicates the operation is only valid for stateless services.
    InvalidForStatefulServices = NativeErrorCode::FABRIC_E_INVALID_FOR_STATEFUL_SERVICES as u32,

    /// Indicates the operation is only valid for stateful services.
    InvalidForStatelessServices = NativeErrorCode::FABRIC_E_INVALID_FOR_STATELESS_SERVICES as u32,

    /// Indicates the operation is only valid for stateful persistent services.
    OnlyValidForStatefulPersistentServices =
        NativeErrorCode::FABRIC_E_ONLY_VALID_FOR_STATEFUL_PERSISTENT_SERVICES as u32,

    /// Indicates the upload session ID is invalid. Plesea use GUID as upload session ID.
    InvalidUploadSessionId = NativeErrorCode::FABRIC_E_INVALID_UPLOAD_SESSION_ID as u32,

    /// Indicates that the backup protection is not enabled
    BackupNotEnabled = NativeErrorCode::FABRIC_E_BACKUP_NOT_ENABLED as u32,

    /// Indicates that there is a backup protection enablement
    BackupEnabled = NativeErrorCode::FABRIC_E_BACKUP_IS_ENABLED as u32,

    /// Indicates the Backup Policy does not Exist
    BackupPolicyDoesNotExist = NativeErrorCode::FABRIC_E_BACKUP_POLICY_DOES_NOT_EXIST as u32,

    /// Indicates the Backup Policy is already Exists
    BackupPolicyAlreayExists = NativeErrorCode::FABRIC_E_BACKUP_POLICY_ALREADY_EXISTS as u32,

    /// Indicates that a partition is already has a restore in progress
    RestoreAlreadyInProgress = NativeErrorCode::FABRIC_E_RESTORE_IN_PROGRESS as u32,

    /// Indicates that the source from where restore is requested has a properties mismatch with target partition
    RestoreSourceTargetPartitionMismatch =
        NativeErrorCode::FABRIC_E_RESTORE_SOURCE_TARGET_PARTITION_MISMATCH as u32,

    /// Indicates the Restore cannot be triggered as Fault Analysis Service is not running
    FaultAnalysisServiceNotEnabled =
        NativeErrorCode::FABRIC_E_FAULT_ANALYSIS_SERVICE_NOT_ENABLED as u32,

    /// Indicates the container is not found.
    ContainerNotFound = NativeErrorCode::FABRIC_E_CONTAINER_NOT_FOUND as u32,

    /// Indicates that the operation is performed on a disposed object.
    ObjectDisposed = NativeErrorCode::FABRIC_E_OBJECT_DISPOSED as u32,

    /// Indicates the partition is not readable.
    NotReadable = NativeErrorCode::FABRIC_E_NOT_READABLE as u32,

    /// Indicates that operation is invalid.
    InvalidOperation = NativeErrorCode::FABRIC_E_INVALID_OPERATION as u32,

    /// Indicates the single instance application already exists.
    SingleInstanceApplicationAlreadyExists =
        NativeErrorCode::FABRIC_E_SINGLE_INSTANCE_APPLICATION_ALREADY_EXISTS as u32,

    /// Indicates the single instance application is not found.
    SingleInstanceApplicationNotFound =
        NativeErrorCode::FABRIC_E_SINGLE_INSTANCE_APPLICATION_NOT_FOUND as u32,

    /// Indicates that the volume already exists.
    VolumeAlreadyExists = NativeErrorCode::FABRIC_E_VOLUME_ALREADY_EXISTS as u32,

    /// Indicates that the volume is not found.
    VolumeNotFound = NativeErrorCode::FABRIC_E_VOLUME_NOT_FOUND as u32,

    /// Indicates the scaling policy specified for the service is invalid.
    /// Verify that every  and  is valid in the context of the kind and metrics of the service.
    InvalidServiceScalingPolicy = NativeErrorCode::FABRIC_E_INVALID_SERVICE_SCALING_POLICY as u32,

    /// Indicates the service is undergoing database migration and writes are currently not available.
    DatabaseMigrationInProgress = NativeErrorCode::FABRIC_E_DATABASE_MIGRATION_IN_PROGRESS as u32,

    /// Indicates generic error happens in central secret service
    CentralSecretServiceGenericError =
        NativeErrorCode::FABRIC_E_CENTRAL_SECRET_SERVICE_GENERIC as u32,

    /// Indicates the compose deployment is not upgrading. Call  to get more information.
    ComposeDeploymentNotUpgrading =
        NativeErrorCode::FABRIC_E_COMPOSE_DEPLOYMENT_NOT_UPGRADING as u32,

    /// Indicates that the secret is invalid
    SecretInvalid = NativeErrorCode::FABRIC_E_SECRET_INVALID as u32,

    /// Indicates that the secret version already exists
    SecretVersionAlreadyExists = NativeErrorCode::FABRIC_E_SECRET_VERSION_ALREADY_EXISTS as u32,

    /// Indicates that the single instance application's upgrade is in progress.
    SingleInstanceApplicationUpgradeInProgress =
        NativeErrorCode::FABRIC_E_SINGLE_INSTANCE_APPLICATION_UPGRADE_IN_PROGRESS as u32,

    /// Indicates that the operation is not supported.
    OperationNotSupported = NativeErrorCode::FABRIC_E_OPERATION_NOT_SUPPORTED as u32,

    /// Indicates that the network is not found.
    NetworkNotFound = NativeErrorCode::FABRIC_E_NETWORK_NOT_FOUND as u32,

    /// Indicates that the network is currently in use.
    NetworkInUse = NativeErrorCode::FABRIC_E_NETWORK_IN_USE as u32,

    EndpointNotReferenced = NativeErrorCode::FABRIC_E_ENDPOINT_NOT_REFERENCED as u32,

    /// Indicates that the Copy has been aborted.
    CopyAborted = NativeErrorCode::FABRIC_E_COPY_ABORTED as u32,

    /// Indicates that RunToCompletion is incompatible with SharedProcess.
    IncompatibleRunToCompletion =
        NativeErrorCode::FABRIC_E_RUN_TO_COMPLETION_INCOMPATIBLE_WITH_SHARED_PROCESS as u32,

    /// Indicates that the version store has reached it's maximum allowed memory usage. Version Store Size can be configured with LocalEseStoreSettings.MaxVerPages.
    VersionStoreOutOfMemory = NativeErrorCode::FABRIC_E_VERSION_STORE_OUT_OF_MEMORY as u32,

    /// Indicates that Backup is not found for the Partition.
    BackupNotFound = NativeErrorCode::FABRIC_E_BACKUP_NOT_FOUND as u32,

    /// Indicates that Auto Restore operation needs to be skipped.
    SkipRestoreOperation = NativeErrorCode::FABRIC_E_SKIP_RESTORE_OPERATION as u32,

    /// Indicates service fabric has used the maximum allowed sessions.
    StoreOutOfSessions = NativeErrorCode::FABRIC_E_STORE_OUT_OF_SESSIONS as u32,

    /// Indicates that Backup Restore service is waiting for user input to conclude restore operation.
    RestoreWaitingForUserIntervention =
        NativeErrorCode::FABRIC_E_RESTORE_WAITING_FOR_USER_INTERVENTION as u32,

    /// Indicates that Service Fabric detected corruption in data stored on disk.
    DatabaseFilesCorrupted = NativeErrorCode::FABRIC_E_DATABASE_FILES_CORRUPTED as u32,

    /// Indicates that a service could not be updated to maximum sensitivity while preserving collocation.
    InsufficientMaxLoadCapacity = NativeErrorCode::FABRIC_E_INSUFFICIENT_MAX_LOAD_CAPACITY as u32,

    /// Indicates that Service Fabric encountered an error when accessing data stored on disk. Disable the node and check the drive for failures.
    StoreDiskError = NativeErrorCode::FABRIC_E_STORE_DISK_ERROR as u32,

    /// Indicates that service is already in requested state during Disable/Enable service operation.
    ServiceAlreadyInRequestedState =
        NativeErrorCode::FABRIC_E_SERVICE_ALREADY_IN_REQUESTED_STATE as u32,

    /// Indicates that Disable/Enable Service feature is disabled.
    DisableEnableServiceFeatureDisabled =
        NativeErrorCode::FABRIC_E_DISABLE_ENABLE_SERVICE_FEATURE_DISABLED as u32,

    /// Indicates that the client has reached a maximum of allowed disabled services in the cluster.
    MaxAllowedDisabledServicesReached =
        NativeErrorCode::FABRIC_E_MAX_ALLOWED_DISABLED_SERVICES_REACHED as u32,

    /// Indicates that the service is disabled.
    ServiceDisabled = NativeErrorCode::FABRIC_E_SERVICE_DISABLED as u32,

    /// Indicates that the service disable is in progress.
    ServiceDisableInProgress = NativeErrorCode::FABRIC_E_SERVICE_DISABLE_IN_PROGRESS as u32,

    /// Indicates that the replica has exceeded the Long Value IDs limit. Compaction is needed.
    StoreOutOfLongValueIDs = NativeErrorCode::FABRIC_E_STORE_OUT_OF_LONG_VALUE_IDS as u32,
}

impl FabricErrorCode {
    /// Async operation is not completed.
    pub const AsyncOperationNotComplete: FabricErrorCode = FabricErrorCode::OperationNotComplete;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum NativeErrorCode {
    // Common HRESULT codes from http://msdn.microsoft.com/en-us/library/aa378137(VS.85).aspx
    S_OK = 0x00000000,
    E_ABORT = 0x80004004,
    E_ACCESSDENIED = 0x80070005,
    E_FAIL = 0x80004005,
    E_HANDLE = 0x80070006,
    E_INVALIDARG = 0x80070057,
    E_NOINTERFACE = 0x80004002,
    E_NOTIMPL = 0x80004001,
    E_OUTOFMEMORY = 0x8007000E,
    E_POINTER = 0x80004003,
    E_UNEXPECTED = 0x8000FFFF,

    // HRESULT codes from win32 errors
    E_FILE_EXISTS = 0x80070050,   // HRESULT_FROM_WIN32(ERROR_FILE_EXISTS)
    E_DIR_NOT_EMPTY = 0x80070091, //  HRESULT_FROM_WIN32(ERROR_DIR_NOT_EMPTY)
    E_NOT_FOUND = 0x80070490,     //  HRESULT_FROM_WIN32(ERROR_NOT_FOUND)

    // HRESULT values of Exceptions which are not in coreCLR

    // HRESULT value for DuplicateWaitObjectException taken from https://msdn.microsoft.com/en-us/library/system.duplicatewaitobjectexception(v=vs.110).aspx
    COR_E_DUPLICATEWAITOBJECT = 0x80131529,
    // HRESULT value for TypeLoadException taken from https://msdn.microsoft.com/en-us/library/system.typeloadexception(v=vs.110).aspx
    COR_E_TYPELOAD = 0x80131522,

    // Fabric codes
    FABRIC_E_COMMUNICATION_ERROR = 0x80071bbc,
    FABRIC_E_INVALID_ADDRESS,
    FABRIC_E_INVALID_NAME_URI,
    FABRIC_E_INVALID_PARTITION_KEY,
    FABRIC_E_NAME_ALREADY_EXISTS,
    FABRIC_E_NAME_DOES_NOT_EXIST,
    FABRIC_E_NAME_NOT_EMPTY,
    FABRIC_E_NODE_NOT_FOUND,
    FABRIC_E_NODE_IS_UP,
    FABRIC_E_NO_WRITE_QUORUM,
    FABRIC_E_NOT_PRIMARY,
    FABRIC_E_NOT_READY,
    FABRIC_E_OPERATION_NOT_COMPLETE,
    FABRIC_E_PROPERTY_DOES_NOT_EXIST,
    FABRIC_E_RECONFIGURATION_PENDING,
    FABRIC_E_REPLICATION_QUEUE_FULL,
    FABRIC_E_SERVICE_ALREADY_EXISTS,
    FABRIC_E_SERVICE_DOES_NOT_EXIST,
    FABRIC_E_SERVICE_OFFLINE,
    FABRIC_E_SERVICE_METADATA_MISMATCH,
    FABRIC_E_SERVICE_AFFINITY_CHAIN_NOT_SUPPORTED,
    FABRIC_E_SERVICE_TYPE_ALREADY_REGISTERED,
    FABRIC_E_SERVICE_TYPE_NOT_REGISTERED,
    FABRIC_E_VALUE_TOO_LARGE,
    FABRIC_E_VALUE_EMPTY,
    FABRIC_E_PROPERTY_CHECK_FAILED,
    FABRIC_E_WRITE_CONFLICT,
    FABRIC_E_ENUMERATION_COMPLETED,
    FABRIC_E_APPLICATION_TYPE_PROVISION_IN_PROGRESS,
    FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS,
    FABRIC_E_APPLICATION_TYPE_NOT_FOUND,
    FABRIC_E_APPLICATION_TYPE_IN_USE,
    FABRIC_E_APPLICATION_ALREADY_EXISTS,
    FABRIC_E_APPLICATION_NOT_FOUND,
    FABRIC_E_APPLICATION_UPGRADE_IN_PROGRESS,
    FABRIC_E_APPLICATION_UPGRADE_VALIDATION_ERROR,
    FABRIC_E_SERVICE_TYPE_NOT_FOUND,
    FABRIC_E_SERVICE_TYPE_MISMATCH,
    FABRIC_E_SERVICE_TYPE_TEMPLATE_NOT_FOUND,
    FABRIC_E_CONFIGURATION_SECTION_NOT_FOUND,
    FABRIC_E_CONFIGURATION_PARAMETER_NOT_FOUND,
    FABRIC_E_INVALID_CONFIGURATION,
    FABRIC_E_IMAGEBUILDER_VALIDATION_ERROR,
    FABRIC_E_PARTITION_NOT_FOUND,
    FABRIC_E_REPLICA_DOES_NOT_EXIST,
    FABRIC_E_SERVICE_GROUP_ALREADY_EXISTS,
    FABRIC_E_SERVICE_GROUP_DOES_NOT_EXIST,
    FABRIC_E_PROCESS_DEACTIVATED,
    FABRIC_E_PROCESS_ABORTED,
    FABRIC_E_UPGRADE_FAILED,
    FABRIC_E_INVALID_CREDENTIAL_TYPE,
    FABRIC_E_INVALID_X509_FIND_TYPE,
    FABRIC_E_INVALID_X509_STORE_LOCATION,
    FABRIC_E_INVALID_X509_STORE_NAME,
    FABRIC_E_INVALID_X509_THUMBPRINT,
    FABRIC_E_INVALID_PROTECTION_LEVEL,
    FABRIC_E_INVALID_X509_STORE,
    FABRIC_E_INVALID_SUBJECT_NAME,
    FABRIC_E_INVALID_ALLOWED_COMMON_NAME_LIST,
    FABRIC_E_INVALID_CREDENTIALS,
    FABRIC_E_DECRYPTION_FAILED,
    FABRIC_E_CONFIGURATION_PACKAGE_NOT_FOUND,
    FABRIC_E_DATA_PACKAGE_NOT_FOUND,
    FABRIC_E_CODE_PACKAGE_NOT_FOUND,
    FABRIC_E_SERVICE_ENDPOINT_RESOURCE_NOT_FOUND,
    FABRIC_E_INVALID_OPERATION,
    FABRIC_E_OBJECT_CLOSED,
    FABRIC_E_TIMEOUT,
    FABRIC_E_FILE_NOT_FOUND,
    FABRIC_E_DIRECTORY_NOT_FOUND,
    FABRIC_E_INVALID_DIRECTORY,
    FABRIC_E_PATH_TOO_LONG,
    FABRIC_E_IMAGESTORE_IOERROR,
    FABRIC_E_CORRUPTED_IMAGE_STORE_OBJECT_FOUND,
    FABRIC_E_APPLICATION_NOT_UPGRADING,
    FABRIC_E_APPLICATION_ALREADY_IN_TARGET_VERSION,
    FABRIC_E_IMAGEBUILDER_UNEXPECTED_ERROR,
    FABRIC_E_FABRIC_VERSION_NOT_FOUND,
    FABRIC_E_FABRIC_VERSION_IN_USE,
    FABRIC_E_FABRIC_VERSION_ALREADY_EXISTS,
    FABRIC_E_FABRIC_ALREADY_IN_TARGET_VERSION,
    FABRIC_E_FABRIC_NOT_UPGRADING,
    FABRIC_E_FABRIC_UPGRADE_IN_PROGRESS,
    FABRIC_E_FABRIC_UPGRADE_VALIDATION_ERROR,
    FABRIC_E_HEALTH_MAX_REPORTS_REACHED,
    FABRIC_E_HEALTH_STALE_REPORT,
    FABRIC_E_KEY_TOO_LARGE,
    FABRIC_E_KEY_NOT_FOUND,
    FABRIC_E_SEQUENCE_NUMBER_CHECK_FAILED,
    FABRIC_E_ENCRYPTION_FAILED,
    FABRIC_E_INVALID_ATOMIC_GROUP,
    FABRIC_E_HEALTH_ENTITY_NOT_FOUND,
    FABRIC_E_SERVICE_MANIFEST_NOT_FOUND,
    FABRIC_E_RELIABLE_SESSION_TRANSPORT_STARTUP_FAILURE,
    FABRIC_E_RELIABLE_SESSION_ALREADY_EXISTS,
    FABRIC_E_RELIABLE_SESSION_CANNOT_CONNECT,
    FABRIC_E_RELIABLE_SESSION_MANAGER_EXISTS,
    FABRIC_E_RELIABLE_SESSION_REJECTED,
    FABRIC_E_RELIABLE_SESSION_MANAGER_ALREADY_LISTENING,
    FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_FOUND,
    FABRIC_E_RELIABLE_SESSION_MANAGER_NOT_LISTENING,
    FABRIC_E_INVALID_SERVICE_TYPE,
    FABRIC_E_IMAGEBUILDER_TIMEOUT,
    FABRIC_E_IMAGEBUILDER_ACCESS_DENIED,
    FABRIC_E_IMAGEBUILDER_INVALID_MSI_FILE,
    FABRIC_E_SERVICE_TOO_BUSY,
    FABRIC_E_TRANSACTION_NOT_ACTIVE,
    FABRIC_E_REPAIR_TASK_ALREADY_EXISTS,
    FABRIC_E_REPAIR_TASK_NOT_FOUND,
    FABRIC_E_RELIABLE_SESSION_NOT_FOUND,
    FABRIC_E_RELIABLE_SESSION_QUEUE_EMPTY,
    FABRIC_E_RELIABLE_SESSION_QUOTA_EXCEEDED,
    FABRIC_E_RELIABLE_SESSION_SERVICE_FAULTED,
    FABRIC_E_RELIABLE_SESSION_INVALID_TARGET_PARTITION,
    FABRIC_E_TRANSACTION_TOO_LARGE,
    FABRIC_E_REPLICATION_OPERATION_TOO_LARGE,
    FABRIC_E_INSTANCE_ID_MISMATCH,
    FABRIC_E_UPGRADE_DOMAIN_ALREADY_COMPLETED,
    FABRIC_E_NODE_HAS_NOT_STOPPED_YET,
    FABRIC_E_INSUFFICIENT_CLUSTER_CAPACITY,
    FABRIC_E_INVALID_PACKAGE_SHARING_POLICY,
    FABRIC_E_PREDEPLOYMENT_NOT_ALLOWED,

    /// <summary>
    /// Invalid backup setting. E.g. incremental backup option is not set upfront etc.
    /// </summary>
    FABRIC_E_INVALID_BACKUP_SETTING,

    /// <summary>
    /// Incremental backups can only be done after an initial full backup.
    /// </summary>
    FABRIC_E_MISSING_FULL_BACKUP,

    /// <summary>
    /// A backup is currently in progress.
    /// </summary>
    FABRIC_E_BACKUP_IN_PROGRESS,

    FABRIC_E_DUPLICATE_SERVICE_NOTIFICATION_FILTER_NAME,
    FABRIC_E_INVALID_REPLICA_OPERATION,
    FABRIC_E_INVALID_REPLICA_STATE,
    FABRIC_E_LOADBALANCER_NOT_READY,
    FABRIC_E_INVALID_PARTITION_OPERATION,
    FABRIC_E_PRIMARY_ALREADY_EXISTS,
    FABRIC_E_SECONDARY_ALREADY_EXISTS,

    /// <summary>
    /// The backup directory is not empty.
    /// </summary>
    FABRIC_E_BACKUP_DIRECTORY_NOT_EMPTY,

    FABRIC_E_FORCE_NOT_SUPPORTED_FOR_REPLICA_OPERATION,

    FABRIC_E_ACQUIRE_FILE_LOCK_FAILED,

    FABRIC_E_CONNECTION_DENIED,
    FABRIC_E_SERVER_AUTHENTICATION_FAILED,

    FABRIC_E_CONSTRAINT_KEY_UNDEFINED,
    FABRIC_E_MULTITHREADED_TRANSACTIONS_NOT_ALLOWED,

    FABRIC_E_INVALID_X509_NAME_LIST,

    FABRIC_E_VERBOSE_FM_PLACEMENT_HEALTH_REPORTING_REQUIRED,

    FABRIC_E_GATEWAY_NOT_REACHABLE,

    FABRIC_E_USER_ROLE_CLIENT_CERTIFICATE_NOT_CONFIGURED,

    FABRIC_E_TRANSACTION_ABORTED,

    FABRIC_E_CANNOT_CONNECT,

    FABRIC_E_MESSAGE_TOO_LARGE,

    FABRIC_E_CONSTRAINT_NOT_SATISFIED,

    FABRIC_E_ENDPOINT_NOT_FOUND,

    FABRIC_E_APPLICATION_UPDATE_IN_PROGRESS,

    /// <summary>
    /// Deletion of backup files/directory failed. Currently this can happen
    /// in a scenario where backup is used mainly to truncate logs.
    /// </summary>
    FABRIC_E_DELETE_BACKUP_FILE_FAILED,

    FABRIC_E_CONNECTION_CLOSED_BY_REMOTE_END,

    FABRIC_E_INVALID_TEST_COMMAND_STATE,

    FABRIC_E_TEST_COMMAND_OPERATION_ID_ALREADY_EXISTS,

    FABRIC_E_CM_OPERATION_FAILED,

    FABRIC_E_IMAGEBUILDER_RESERVED_DIRECTORY_ERROR,

    FABRIC_E_CERTIFICATE_NOT_FOUND,

    FABRIC_E_CHAOS_ALREADY_RUNNING,

    FABRIC_E_FABRIC_DATA_ROOT_NOT_FOUND,

    FABRIC_E_INVALID_RESTORE_DATA,

    FABRIC_E_DUPLICATE_BACKUPS,

    FABRIC_E_INVALID_BACKUP_CHAIN,

    FABRIC_E_STOP_IN_PROGRESS,

    FABRIC_E_ALREADY_STOPPED,

    FABRIC_E_NODE_IS_DOWN,

    FABRIC_E_NODE_TRANSITION_IN_PROGRESS,

    FABRIC_E_INVALID_BACKUP,

    FABRIC_E_INVALID_INSTANCE_ID,

    FABRIC_E_INVALID_DURATION,

    FABRIC_E_RESTORE_SAFE_CHECK_FAILED,

    FABRIC_E_CONFIG_UPGRADE_FAILED,

    FABRIC_E_UPLOAD_SESSION_RANGE_NOT_SATISFIABLE,

    FABRIC_E_UPLOAD_SESSION_ID_CONFLICT,

    FABRIC_E_INVALID_PARTITION_SELECTOR,

    FABRIC_E_INVALID_REPLICA_SELECTOR,

    FABRIC_E_DNS_SERVICE_NOT_FOUND,

    FABRIC_E_INVALID_DNS_NAME,

    FABRIC_E_DNS_NAME_IN_USE,

    FABRIC_E_COMPOSE_DEPLOYMENT_ALREADY_EXISTS,

    FABRIC_E_COMPOSE_DEPLOYMENT_NOT_FOUND,

    FABRIC_E_INVALID_FOR_STATEFUL_SERVICES,

    FABRIC_E_INVALID_FOR_STATELESS_SERVICES,

    FABRIC_E_ONLY_VALID_FOR_STATEFUL_PERSISTENT_SERVICES,

    FABRIC_E_INVALID_UPLOAD_SESSION_ID,

    FABRIC_E_BACKUP_NOT_ENABLED,

    FABRIC_E_BACKUP_IS_ENABLED,

    FABRIC_E_BACKUP_POLICY_DOES_NOT_EXIST,

    FABRIC_E_BACKUP_POLICY_ALREADY_EXISTS,

    FABRIC_E_RESTORE_IN_PROGRESS,

    FABRIC_E_RESTORE_SOURCE_TARGET_PARTITION_MISMATCH,

    FABRIC_E_FAULT_ANALYSIS_SERVICE_NOT_ENABLED,

    FABRIC_E_CONTAINER_NOT_FOUND,

    FABRIC_E_OBJECT_DISPOSED,

    FABRIC_E_NOT_READABLE,

    FABRIC_E_BACKUPCOPIER_UNEXPECTED_ERROR,

    FABRIC_E_BACKUPCOPIER_TIMEOUT,

    FABRIC_E_BACKUPCOPIER_ACCESS_DENIED,

    FABRIC_E_INVALID_SERVICE_SCALING_POLICY,

    FABRIC_E_SINGLE_INSTANCE_APPLICATION_ALREADY_EXISTS,

    FABRIC_E_SINGLE_INSTANCE_APPLICATION_NOT_FOUND,

    FABRIC_E_VOLUME_ALREADY_EXISTS,

    FABRIC_E_VOLUME_NOT_FOUND,

    FABRIC_E_DATABASE_MIGRATION_IN_PROGRESS,

    FABRIC_E_CENTRAL_SECRET_SERVICE_GENERIC,

    FABRIC_E_SECRET_INVALID,

    FABRIC_E_SECRET_VERSION_ALREADY_EXISTS,

    FABRIC_E_SINGLE_INSTANCE_APPLICATION_UPGRADE_IN_PROGRESS,

    FABRIC_E_OPERATION_NOT_SUPPORTED,

    FABRIC_E_COMPOSE_DEPLOYMENT_NOT_UPGRADING,

    FABRIC_E_SECRET_TYPE_CANNOT_BE_CHANGED,

    FABRIC_E_NETWORK_NOT_FOUND,

    FABRIC_E_NETWORK_IN_USE,

    FABRIC_E_ENDPOINT_NOT_REFERENCED,

    FABRIC_E_INSTANCE_ALREADY_EXISTS = 0x87b00000,

    FABRIC_E_NODE_TYPE_NOT_FOUND = NativeErrorCode::FABRIC_E_INSTANCE_ALREADY_EXISTS as u32 + 1,

    FABRIC_E_INSTANCE_COUNT_UPDATE_NOT_ALLOWED =
        NativeErrorCode::FABRIC_E_NODE_TYPE_NOT_FOUND as u32 + 1,

    FABRIC_E_COPY_ABORTED = NativeErrorCode::FABRIC_E_INSTANCE_COUNT_UPDATE_NOT_ALLOWED as u32 + 1,

    FABRIC_E_AUXILIARY_ALREADY_EXISTS = NativeErrorCode::FABRIC_E_COPY_ABORTED as u32 + 1,

    FABRIC_E_AUXILIARY_FEATURE_DISABLED =
        NativeErrorCode::FABRIC_E_AUXILIARY_ALREADY_EXISTS as u32 + 1,

    FABRIC_E_RUN_TO_COMPLETION_INCOMPATIBLE_WITH_SHARED_PROCESS =
        NativeErrorCode::FABRIC_E_AUXILIARY_FEATURE_DISABLED as u32 + 1,

    FABRIC_E_VERSION_STORE_OUT_OF_MEMORY =
        NativeErrorCode::FABRIC_E_RUN_TO_COMPLETION_INCOMPATIBLE_WITH_SHARED_PROCESS as u32 + 1,

    /// <summary>
    /// No Backup is found for a partition.
    /// </summary>
    FABRIC_E_BACKUP_NOT_FOUND = NativeErrorCode::FABRIC_E_VERSION_STORE_OUT_OF_MEMORY as u32 + 1,

    /// <summary>
    /// Skip Restore Operation
    /// </summary>
    FABRIC_E_SKIP_RESTORE_OPERATION = NativeErrorCode::FABRIC_E_BACKUP_NOT_FOUND as u32 + 1,

    FABRIC_E_STORE_OUT_OF_SESSIONS = NativeErrorCode::FABRIC_E_SKIP_RESTORE_OPERATION as u32 + 1,
    /// <summary>
    /// Restore Waiting For User Input
    /// </summary>
    FABRIC_E_RESTORE_WAITING_FOR_USER_INTERVENTION =
        NativeErrorCode::FABRIC_E_STORE_OUT_OF_SESSIONS as u32 + 1,

    FABRIC_E_DATABASE_FILES_CORRUPTED =
        NativeErrorCode::FABRIC_E_RESTORE_WAITING_FOR_USER_INTERVENTION as u32 + 1,

    FABRIC_E_INSUFFICIENT_MAX_LOAD_CAPACITY =
        NativeErrorCode::FABRIC_E_DATABASE_FILES_CORRUPTED as u32 + 1,

    FABRIC_E_STORE_DISK_ERROR = NativeErrorCode::FABRIC_E_INSUFFICIENT_MAX_LOAD_CAPACITY as u32 + 1,

    FABRIC_E_SERVICE_ALREADY_IN_REQUESTED_STATE =
        NativeErrorCode::FABRIC_E_STORE_DISK_ERROR as u32 + 1,

    FABRIC_E_DISABLE_ENABLE_SERVICE_FEATURE_DISABLED =
        NativeErrorCode::FABRIC_E_SERVICE_ALREADY_IN_REQUESTED_STATE as u32 + 1,

    FABRIC_E_MAX_ALLOWED_DISABLED_SERVICES_REACHED =
        NativeErrorCode::FABRIC_E_DISABLE_ENABLE_SERVICE_FEATURE_DISABLED as u32 + 1,

    FABRIC_E_SERVICE_DISABLED =
        NativeErrorCode::FABRIC_E_MAX_ALLOWED_DISABLED_SERVICES_REACHED as u32 + 1,

    FABRIC_E_SERVICE_DISABLE_IN_PROGRESS = NativeErrorCode::FABRIC_E_SERVICE_DISABLED as u32 + 1,

    FABRIC_E_STORE_OUT_OF_LONG_VALUE_IDS =
        NativeErrorCode::FABRIC_E_SERVICE_DISABLE_IN_PROGRESS as u32 + 1,

    FABRIC_E_FACILITY_SF_LAST_HRESULT = 0x87b0ffff,
}

impl NativeErrorCode {
    pub const FABRIC_E_FIRST_RESERVED_ERROR_CODE: u32 = 0x80071bbc;
    pub const FABRIC_E_LAST_RESERVED_ERROR_CODE: u32 = 0x80071d4b;
}
