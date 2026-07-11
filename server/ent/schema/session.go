package schema

import (
	"time"

	"entgo.io/ent"
	"entgo.io/ent/schema/edge"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/index"

	"github.com/google/uuid"
)

type Session struct {
	ent.Schema
}

func (Session) Fields() []ent.Field {
	return []ent.Field{
		field.UUID("id", uuid.UUID{}).
			Default(uuid.New).
			Immutable(),

		field.UUID("user_id", uuid.UUID{}),

		field.Time("expires_at"),

		field.Time("revoked_at").
			Optional().
			Nillable(),

		field.String("ip_address").
			Optional().
			Nillable(),

		field.String("user_agent").
			Optional().
			Nillable(),

		field.String("device").
			Optional().
			Nillable(),

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

func (Session) Edges() []ent.Edge {
	return []ent.Edge{
		edge.To("user", User.Type).Unique().Required(),
	}
}

func (Session) Indexes() []ent.Index {
	return []ent.Index{
		index.Fields("user_id").
			Edges("user").
			Unique(),
	}
}
