package schema

import (
	"regexp"
	"time"

	"entgo.io/ent"
	"entgo.io/ent/schema/field"
	"github.com/google/uuid"
)

type Game struct {
	ent.Schema
}

func (Game) Fields() []ent.Field {
	return []ent.Field{
		field.UUID("id", uuid.UUID{}).
			Default(uuid.New).
			Immutable(),

		field.String("name").
			MinLen(1).
			MaxLen(128),

		field.String("slug").
			MinLen(1).
			MaxLen(128).
			Match(regexp.MustCompile(`^[a-z0-9_-]+$`)).
			Unique(),

		field.Int("project_count").
			Default(0),

		field.String("card_url").
			Optional().
			Nillable(),

		field.String("banner_url").
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

func (Game) Edges() []ent.Edge {
	return nil
}
