/**
 * The BACKEND_URL value defaults to empty string because in production
 * the backend and frontend are hosted on the same server, so all of the
 * route paths are relative. The environment variable REACT_APP_BACKEND_URL
 * can be used to override this setting for use in development.
 */
export const BACKEND_URL = process.env["REACT_APP_BACKEND_URL"] || "";
