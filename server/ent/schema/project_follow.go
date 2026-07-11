package schema

import (
	"time"

	"entgo.io/ent"
	"entgo.io/ent/schema/edge"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/index"

	"github.com/google/uuid"
)

type ProjectFollow struct {
	ent.Schema
}

func (ProjectFollow) Fields() []ent.Field {
	return []ent.Field{
		field.UUID("id", uuid.UUID{}).
			Default(uuid.New).
			Immutable(),

		field.UUID("project_id", uuid.UUID{}),

		field.UUID("user_id", uuid.UUID{}),

		field.Time("created_at").
			Default(time.Now).
			Immutable(),

		field.Time("updated_at").
			Default(time.Now).
			UpdateDefault(time.Now),
	}
}

func (ProjectFollow) Edges() []ent.Edge {
	return []ent.Edge{
		edge.To("project", Project.Type).
			Field("project_id").
			Unique().
			Required(),

		edge.To("user", User.Type).
			Field("user_id").
			Unique().
			Required(),
	}
}

func (ProjectFollow) Indexes() []ent.Index {
	return []ent.Index{
		index.Fields("project_id", "user_id").Unique(),
	}
}
