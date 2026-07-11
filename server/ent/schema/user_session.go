package schema

import (
	"time"

	"entgo.io/ent"
	"entgo.io/ent/schema/field"
	"github.com/google/uuid"
)

type UserSession struct {
	ent.Schema
}

func (UserSession) Fields() []ent.Field {
	return []ent.Field{
		field.UUID("id", uuid.UUID{}).
			Default(uuid.New).
			Immutable(),

		field.UUID("user_id", uuid.UUID{}),

		field.String("session_token").
			MinLen(1).
			MaxLen(128).
			Unique(),

		field.String("ip_address").
			Optional().
			Nillable(),

		field.String("user_agent").
			Optional().
			Nillable(),

		field.String("expires_at").
			Optional().
			Nillable(),

		field.String("last_accessed_at").
			Optional().
			Nillable(),

		field.Time("created_at").
			Default(time.Now).
			Immutable(),

		field.Time("updated_at").
			Default(time.Now).
			UpdateDefault(time.Now),
	}
}

func (UserSession) Edges() []ent.Edge {
	return nil
}
