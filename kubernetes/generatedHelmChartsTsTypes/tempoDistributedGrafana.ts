// Don't Edit. This is autogenerated.
export interface ITempoDistributedGrafana {
    global: Global;
    fullnameOverride: string;
    tempo: Tempo;
    serviceAccount: ServiceAccount;
    rbac: Rbac;
    ingester: Ingester;
    metricsGenerator: MetricsGenerator;
    distributor: Distributor;
    compactor: Compactor;
    querier: Querier;
    queryFrontend: QueryFrontend;
    search: Search;
    multitenancyEnabled: boolean;
    traces: Traces;
    config: string;
    server: Server;
    storage: Storage2;
    global_overrides: Globaloverrides;
    overrides: string;
    memcached: Memcached;
    memcachedExporter: MemcachedExporter;
    serviceMonitor: ServiceMonitor;
    prometheusRule: PrometheusRule;
    gateway: Gateway;
}
interface Gateway {
    enabled: boolean;
    replicas: number;
    autoscaling: Autoscaling;
    verboseLogging: boolean;
    image: Image5;
    priorityClassName?: any;
    podLabels: PodLabels;
    podAnnotations: PodLabels;
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    extraVolumes: any[];
    extraVolumeMounts: any[];
    resources: PodLabels;
    terminationGracePeriodSeconds: number;
    affinity: string;
    nodeSelector: PodLabels;
    tolerations: any[];
    service: Service3;
    ingress: Ingress;
    basicAuth: BasicAuth;
    readinessProbe: ReadinessProbe;
    nginxConfig: NginxConfig;
}
interface NginxConfig {
    logFormat: string;
    serverSnippet: string;
    httpSnippet: string;
    file: string;
}
interface BasicAuth {
    enabled: boolean;
    username?: any;
    password?: any;
    htpasswd: string;
    existingSecret?: any;
}
interface Ingress {
    enabled: boolean;
    annotations: PodLabels;
    hosts: Host[];
    tls: Tl[];
}
interface Tl {
    secretName: string;
    hosts: string[];
}
interface Host {
    host: string;
    paths: Path[];
}
interface Path {
    path: string;
}
interface Service3 {
    port: number;
    type: string;
    clusterIP?: any;
    nodePort?: any;
    loadBalancerIP?: any;
    annotations: PodLabels;
    labels: PodLabels;
}
interface PrometheusRule {
    enabled: boolean;
    namespace?: any;
    annotations: PodLabels;
    labels: PodLabels;
    groups: any[];
}
interface ServiceMonitor {
    enabled: boolean;
    namespace?: any;
    namespaceSelector: PodLabels;
    annotations: PodLabels;
    labels: PodLabels;
    interval?: any;
    scrapeTimeout?: any;
    relabelings: any[];
    metricRelabelings: any[];
    scheme: string;
    tlsConfig?: any;
}
interface MemcachedExporter {
    enabled: boolean;
    image: Image5;
    resources: PodLabels;
}
interface Memcached {
    enabled: boolean;
    image: Image5;
    host: string;
    replicas: number;
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    podLabels: PodLabels;
    podAnnotations: PodLabels;
    resources: PodLabels;
    affinity: string;
    service: Service;
}
interface Image5 {
    registry?: any;
    repository: string;
    tag: string;
    pullPolicy: string;
}
interface Globaloverrides {
    per_tenant_override_config: string;
}
interface Storage2 {
    trace: Trace;
}
interface Trace {
    backend: string;
}
interface Server {
    httpListenPort: number;
    logLevel: string;
    logFormat: string;
    grpc_server_max_recv_msg_size: number;
    grpc_server_max_send_msg_size: number;
}
interface Traces {
    jaeger: Jaeger;
    zipkin: Grpc;
    otlp: Otlp;
    opencensus: Grpc;
    kafka: PodLabels;
}
interface Otlp {
    http: Grpc;
    grpc: Grpc;
}
interface Jaeger {
    grpc: Grpc;
    thriftBinary: Grpc;
    thriftCompact: Grpc;
    thriftHttp: Grpc;
}
interface Grpc {
    enabled: boolean;
    receiverConfig: PodLabels;
}
interface Search {
    enabled: boolean;
}
interface QueryFrontend {
    query: Query;
    replicas: number;
    autoscaling: Autoscaling;
    image: Image3;
    service: Service2;
    serviceDiscovery: Service;
    priorityClassName?: any;
    podLabels: PodLabels;
    podAnnotations: PodLabels;
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    resources: PodLabels;
    terminationGracePeriodSeconds: number;
    affinity: string;
    nodeSelector: PodLabels;
    tolerations: any[];
    extraVolumeMounts: any[];
    extraVolumes: any[];
}
interface Query {
    enabled: boolean;
    image: Image4;
    resources: PodLabels;
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    extraVolumeMounts: any[];
    extraVolumes: any[];
    config: string;
}
interface Image4 {
    registry?: any;
    repository: string;
    tag?: any;
}
interface Querier {
    replicas: number;
    image: Image3;
    priorityClassName?: any;
    podLabels: PodLabels;
    podAnnotations: PodLabels;
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    resources: PodLabels;
    terminationGracePeriodSeconds: number;
    affinity: string;
    nodeSelector: PodLabels;
    tolerations: any[];
    extraVolumeMounts: any[];
    extraVolumes: any[];
    config: Config5;
    service: Service;
}
interface Config5 {
    frontend_worker: Frontendworker;
}
interface Frontendworker {
    grpc_client_config: PodLabels;
}
interface Compactor {
    replicas: number;
    image: Image3;
    priorityClassName?: any;
    podLabels: PodLabels;
    podAnnotations: PodLabels;
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    resources: PodLabels;
    terminationGracePeriodSeconds: number;
    nodeSelector: PodLabels;
    tolerations: any[];
    extraVolumeMounts: any[];
    extraVolumes: any[];
    config: Config4;
    service: Service;
}
interface Config4 {
    compaction: Compaction;
}
interface Compaction {
    block_retention: string;
}
interface Distributor {
    replicas: number;
    autoscaling: Autoscaling;
    image: Image3;
    service: Service2;
    priorityClassName?: any;
    podLabels: PodLabels;
    podAnnotations: PodLabels;
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    resources: PodLabels;
    terminationGracePeriodSeconds: number;
    affinity: string;
    nodeSelector: PodLabels;
    tolerations: any[];
    extraVolumeMounts: any[];
    extraVolumes: any[];
    config: Config3;
}
interface Config3 {
    log_received_traces?: any;
    log_received_spans: Logreceivedspans;
    extend_writes?: any;
    search_tags_deny_list: any[];
}
interface Logreceivedspans {
    enabled: boolean;
    include_all_attributes: boolean;
    filter_by_status_error: boolean;
}
interface Service2 {
    annotations: PodLabels;
    type: string;
    loadBalancerIP: string;
    loadBalancerSourceRanges: any[];
}
interface MetricsGenerator {
    enabled: boolean;
    annotations: PodLabels;
    replicas: number;
    image: Image3;
    priorityClassName?: any;
    podLabels: PodLabels;
    podAnnotations: PodLabels;
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    resources: PodLabels;
    terminationGracePeriodSeconds: number;
    affinity: string;
    nodeSelector: PodLabels;
    tolerations: any[];
    extraVolumeMounts: any[];
    extraVolumes: any[];
    ports: Port[];
    config: Config2;
    service: Service;
}
interface Config2 {
    registry: Registry;
    processor: Processor;
    storage: Storage;
}
interface Storage {
    path: string;
    wal?: any;
    remote_write_flush_deadline: string;
    remote_write: any[];
}
interface Processor {
    service_graphs: Servicegraphs;
    span_metrics: Spanmetrics;
}
interface Spanmetrics {
    dimensions: any[];
    histogram_buckets: number[];
}
interface Servicegraphs {
    dimensions: any[];
    histogram_buckets: number[];
    max_items: number;
    wait: string;
    workers: number;
}
interface Registry {
    collection_interval: string;
    external_labels: PodLabels;
    stale_duration: string;
}
interface Port {
    name: string;
    port: number;
    service: boolean;
}
interface Ingester {
    annotations: PodLabels;
    replicas: number;
    autoscaling: Autoscaling;
    image: Image3;
    priorityClassName?: any;
    podLabels: PodLabels;
    podAnnotations: PodLabels;
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    resources: PodLabels;
    terminationGracePeriodSeconds: number;
    affinity: string;
    nodeSelector: PodLabels;
    tolerations: any[];
    extraVolumeMounts: any[];
    extraVolumes: any[];
    persistence: Persistence;
    config: Config;
    service: Service;
}
interface Service {
    annotations: PodLabels;
}
interface Config {
    replication_factor: number;
    trace_idle_period?: any;
    flush_check_period?: any;
    max_block_bytes?: any;
    max_block_duration?: any;
    complete_block_timeout?: any;
}
interface Persistence {
    enabled: boolean;
    inMemory: boolean;
    size: string;
    storageClass?: any;
    annotations: PodLabels;
}
interface Image3 {
    registry?: any;
    repository?: any;
    tag?: any;
}
interface Autoscaling {
    enabled: boolean;
    minReplicas: number;
    maxReplicas: number;
    targetCPUUtilizationPercentage: number;
    targetMemoryUtilizationPercentage?: any;
}
interface Rbac {
    create: boolean;
    pspEnabled: boolean;
}
interface ServiceAccount {
    create: boolean;
    name?: any;
    imagePullSecrets: any[];
    annotations: PodLabels;
}
interface Tempo {
    image: Image2;
    readinessProbe: ReadinessProbe;
    podLabels: PodLabels;
    podAnnotations: PodLabels;
    securityContext: SecurityContext;
    podSecurityContext: PodSecurityContext;
    structuredConfig: PodLabels;
}
interface PodSecurityContext {
    fsGroup: number;
}
interface SecurityContext {
    runAsNonRoot: boolean;
    runAsUser: number;
    runAsGroup: number;
    allowPrivilegeEscalation: boolean;
    capabilities: Capabilities;
    readOnlyRootFilesystem: boolean;
}
interface Capabilities {
    drop: string[];
}
interface PodLabels {}
interface ReadinessProbe {
    httpGet: HttpGet;
    initialDelaySeconds: number;
    timeoutSeconds: number;
}
interface HttpGet {
    path: string;
    port: string;
}
interface Image2 {
    registry: string;
    repository: string;
    tag?: any;
    pullPolicy: string;
}
interface Global {
    image: Image;
    priorityClassName?: any;
    clusterDomain: string;
    dnsService: string;
    dnsNamespace: string;
}
interface Image {
    registry: string;
}
