export interface ICertmanagerbitnami {
    global: Global;
    kubeVersion: string;
    nameOverride: string;
    fullnameOverride: string;
    commonLabels: CommonLabels;
    commonAnnotations: CommonLabels;
    extraDeploy: any[];
    logLevel: number;
    clusterResourceNamespace: string;
    leaderElection: LeaderElection;
    installCRDs: boolean;
    replicaCount: number;
    controller: Controller;
    webhook: Webhook;
    cainjector: Cainjector;
    metrics: Metrics;
    rbac: Rbac;
}
interface Rbac {
    create: boolean;
}
interface Metrics {
    enabled: boolean;
    podAnnotations: PodAnnotations;
    serviceMonitor: ServiceMonitor;
}
interface ServiceMonitor {
    path: string;
    targetPort: number;
    enabled: boolean;
    namespace: string;
    jobLabel: string;
    interval: string;
    scrapeTimeout: string;
    relabelings: any[];
    metricRelabelings: any[];
    selector: CommonLabels;
    labels: CommonLabels;
    additionalLabels: CommonLabels;
    honorLabels: boolean;
}
interface PodAnnotations {
    'prometheus.io/path': string;
    'prometheus.io/scrape': string;
    'prometheus.io/port': string;
}
interface Cainjector {
    replicaCount: number;
    image: Image;
    resources: Resources;
    podSecurityContext: PodSecurityContext;
    containerSecurityContext: ContainerSecurityContext;
    podAffinityPreset: string;
    podAntiAffinityPreset: string;
    nodeAffinityPreset: NodeAffinityPreset;
    affinity: CommonLabels;
    nodeSelector: CommonLabels;
    command: any[];
    args: any[];
    priorityClassName: string;
    runtimeClassName: string;
    schedulerName: string;
    topologySpreadConstraints: any[];
    hostAliases: any[];
    tolerations: any[];
    podLabels: CommonLabels;
    podAnnotations: CommonLabels;
    lifecycleHooks: CommonLabels;
    updateStrategy: UpdateStrategy;
    extraEnvVars: any[];
    extraEnvVarsCM: string;
    extraEnvVarsSecret: string;
    extraVolumes: any[];
    extraVolumeMounts: any[];
    initContainers: any[];
    sidecars: any[];
    serviceAccount: ServiceAccount;
}
interface Webhook {
    replicaCount: number;
    image: Image;
    resources: Resources;
    podSecurityContext: PodSecurityContext;
    containerSecurityContext: ContainerSecurityContext;
    podAffinityPreset: string;
    podAntiAffinityPreset: string;
    nodeAffinityPreset: NodeAffinityPreset;
    affinity: CommonLabels;
    nodeSelector: CommonLabels;
    containerPort: number;
    httpsPort: number;
    command: any[];
    args: any[];
    livenessProbe: LivenessProbe;
    readinessProbe: LivenessProbe;
    customStartupProbe: CommonLabels;
    customLivenessProbe: CommonLabels;
    customReadinessProbe: CommonLabels;
    priorityClassName: string;
    runtimeClassName: string;
    schedulerName: string;
    topologySpreadConstraints: any[];
    hostAliases: any[];
    tolerations: any[];
    podLabels: CommonLabels;
    podAnnotations: CommonLabels;
    lifecycleHooks: CommonLabels;
    updateStrategy: UpdateStrategy;
    extraEnvVars: any[];
    extraEnvVarsCM: string;
    extraEnvVarsSecret: string;
    extraVolumes: any[];
    extraVolumeMounts: any[];
    initContainers: any[];
    sidecars: any[];
    serviceAccount: ServiceAccount;
}
interface LivenessProbe {
    enabled: boolean;
    path: string;
    initialDelaySeconds: number;
    periodSeconds: number;
    timeoutSeconds: number;
    successThreshold: number;
    failureThreshold: number;
}
interface Controller {
    replicaCount: number;
    image: Image;
    acmesolver: Acmesolver;
    resources: Resources;
    podSecurityContext: PodSecurityContext;
    containerSecurityContext: ContainerSecurityContext;
    podAffinityPreset: string;
    podAntiAffinityPreset: string;
    nodeAffinityPreset: NodeAffinityPreset;
    affinity: CommonLabels;
    nodeSelector: CommonLabels;
    containerPort: number;
    command: any[];
    args: any[];
    priorityClassName: string;
    runtimeClassName: string;
    schedulerName: string;
    topologySpreadConstraints: any[];
    hostAliases: any[];
    tolerations: any[];
    podLabels: CommonLabels;
    podAnnotations: CommonLabels;
    dnsPolicy: string;
    dnsConfig: CommonLabels;
    lifecycleHooks: CommonLabels;
    updateStrategy: UpdateStrategy;
    extraEnvVars: any[];
    extraEnvVarsCM: string;
    extraEnvVarsSecret: string;
    extraVolumes: any[];
    extraVolumeMounts: any[];
    initContainers: any[];
    sidecars: any[];
    serviceAccount: ServiceAccount;
}
interface ServiceAccount {
    create: boolean;
    name: string;
    annotations: CommonLabels;
    automountServiceAccountToken: boolean;
}
interface UpdateStrategy {
    type: string;
    rollingUpdate: CommonLabels;
}
interface NodeAffinityPreset {
    type: string;
    key: string;
    values: any[];
}
interface ContainerSecurityContext {
    enabled: boolean;
    runAsUser: number;
    runAsNonRoot: boolean;
}
interface PodSecurityContext {
    enabled: boolean;
    fsGroup: number;
}
interface Resources {
    limits: CommonLabels;
    requests: CommonLabels;
}
interface Acmesolver {
    image: Image;
}
interface Image {
    registry: string;
    repository: string;
    tag: string;
    pullPolicy: string;
    pullSecrets: any[];
    debug: boolean;
}
interface LeaderElection {
    namespace: string;
}
interface CommonLabels {}
interface Global {
    imageRegistry: string;
    imagePullSecrets: any[];
    storageClass: string;
}
