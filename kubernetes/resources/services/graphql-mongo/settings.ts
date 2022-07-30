import { AppConfigs } from '../../types/own-types';
import { getEnvironmentVariables } from '../../shared/validations';
import { getBaseUrl } from '../../infrastructure/ingress/hosts';
import { PlainSecretJsonConfig } from '../../../scripts/utils/plainSecretJsonConfig';

const environment = getEnvironmentVariables().ENVIRONMENT;
const secretsFromLocalConfigs = new PlainSecretJsonConfig('graphql-mongo', environment).getSecretsBase64();

export const graphqlMongoSettings: AppConfigs<'graphql-mongo', 'mongodb', 'applications'> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
        replicaCount: 2,
        readinessProbePort: 8000,
        host: '0.0.0.0',
        image: `ghcr.io/oyelowo/graphql-mongo:${getEnvironmentVariables().IMAGE_TAG_GRAPHQL_MONGO}`,
    },

    envVars: {
        APP_ENVIRONMENT: environment,
        APP_HOST: '0.0.0.0',
        APP_PORT: '8000',
        APP_EXTERNAL_BASE_URL: getBaseUrl(environment),
        OAUTH_GITHUB_CLIENT_ID: secretsFromLocalConfigs.GITHUB_CLIENT_ID,
        OAUTH_GITHUB_CLIENT_SECRET: secretsFromLocalConfigs.GITHUB_CLIENT_SECRET,
        OAUTH_GOOGLE_CLIENT_ID: secretsFromLocalConfigs.GOOGLE_CLIENT_ID,
        OAUTH_GOOGLE_CLIENT_SECRET: secretsFromLocalConfigs.GOOGLE_CLIENT_SECRET,

        MONGODB_NAME: 'graphql-mongo-database',
        MONGODB_USERNAME: secretsFromLocalConfigs.MONGODB_USERNAME,
        MONGODB_PASSWORD: secretsFromLocalConfigs.MONGODB_PASSWORD,
        MONGODB_ROOT_USERNAME: secretsFromLocalConfigs.MONGODB_ROOT_USERNAME,
        MONGODB_ROOT_PASSWORD: secretsFromLocalConfigs.MONGODB_ROOT_PASSWORD,
        MONGODB_HOST: 'graphql-mongo-database.applications',
        MONGODB_SERVICE_NAME: 'graphql-mongo-database',
        MONGODB_STORAGE_CLASS: 'linode-block-storage-retain',
        MONGODB_PORT: '27017',

        REDIS_USERNAME: secretsFromLocalConfigs.REDIS_USERNAME,
        REDIS_PASSWORD: secretsFromLocalConfigs.REDIS_PASSWORD,
        REDIS_HOST: 'graphql-mongo-redis-master.applications',
        REDIS_SERVICE_NAME: 'graphql-mongo-redis', // helm chart adds suffix to the name e.g (master) which the rust application must use as above
        REDIS_SERVICE_NAME_WITH_SUFFIX: 'graphql-mongo-redis-master',
        REDIS_PORT: '6379',
    },
    metadata: {
        name: 'graphql-mongo',
        namespace: 'applications',
    },
};
