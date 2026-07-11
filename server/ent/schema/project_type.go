package schema

import (
	"regexp"
	"time"

	"entgo.io/ent"
	"entgo.io/ent/schema/edge"
	"entgo.io/ent/schema/field"
	"github.com/google/uuid"
)

type ProjectType struct {
	ent.Schema
}

func (ProjectType) Fields() []ent.Field {
	return []ent.Field{
		field.UUID("id", uuid.UUID{}).
			Default(uuid.New).
			Immutable(),

		field.UUID("game_id", uuid.UUID{}),

		field.String("name").
			MinLen(1).
			MaxLen(128),

		field.String("slug").
			MinLen(1).
			MaxLen(128).
			Match(regexp.MustCompile(`^[a-z0-9_-]+$`)).
			Unique(),

		field.Time("created_at").
			Default(time.Now).
			Immutable(),

		field.Time("updated_at").
			Default(time.Now).
			UpdateDefault(time.Now),
	}
}

func (ProjectType) Edges() []ent.Edge {
	return []ent.Edge{
		edge.To("game", Game.Type).
			Field("game_id").
			Unique().
			Required(),
	}
}
