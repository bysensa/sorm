import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();

export const nginxIngressProvider = getResourceProvider({
    outputDirectory: `infrastructure/nginx-ingress`,
    environment: ENVIRONMENT,
});
