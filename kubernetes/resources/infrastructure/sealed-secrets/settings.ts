import { getResourceProperties } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
export const sealedSecretsProperties = getResourceProperties("sealed-secrets", ENVIRONMENT);
