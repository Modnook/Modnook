package schema

import (
	"time"

	"entgo.io/ent"
	"entgo.io/ent/schema/edge"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/index"

	"github.com/google/uuid"
)

type UserIdentity struct {
	ent.Schema
}

func (UserIdentity) Fields() []ent.Field {
	return []ent.Field{
		field.UUID("id", uuid.UUID{}).
			Default(uuid.New).
			Immutable(),

		field.UUID("user_id", uuid.UUID{}),

		field.String("provider"),

		field.String("provider_user_id"),

		field.String("email").Optional().Nillable(),

		field.JSON("metadata", map[string]interface{}{}).
			Optional(),

		field.Time("created_at").
			Default(time.Now).
			Immutable(),

		field.Time("updated_at").
			Default(time.Now).
			UpdateDefault(time.Now),
	}
}

func (UserIdentity) Edges() []ent.Edge {
	return []ent.Edge{
		edge.To("user", User.Type).Unique().Required(),
	}
}

func (UserIdentity) Indexes() []ent.Index {
	return []ent.Index{
		index.Fields("provider", "provider_user_id").Unique(),
		index.Fields("user_id"),
	}
}
