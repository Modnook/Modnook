package api

// Generation order: common has no cross-file deps, so it goes first.
// organizations depends on users; projects depends on organizations + users;
// notifications depends on projects + users + organizations. oapi-codegen
// doesn't strictly require this order since each spec file is generated
// independently against the source YAML (not the generated Go), but keeping
// it dependency-ordered here makes the list easier to read.

//go:generate go run github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen --config=generated/common/config-common.yaml openapi/common.yaml
//go:generate go run github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen --config=generated/users/config-users.yaml openapi/users.yaml
//go:generate go run github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen --config=generated/auth/config-auth.yaml openapi/auth.yaml
//go:generate go run github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen --config=generated/games/config-games.yaml openapi/games.yaml
//go:generate go run github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen --config=generated/organizations/config-organizations.yaml openapi/organizations.yaml
//go:generate go run github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen --config=generated/projects/config-projects.yaml openapi/projects.yaml
//go:generate go run github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen --config=generated/notifications/config-notifications.yaml openapi/notifications.yaml
