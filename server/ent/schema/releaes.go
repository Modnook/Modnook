package schema

import (
	"time"

	"entgo.io/ent"
	"entgo.io/ent/schema/edge"
	"entgo.io/ent/schema/field"
	"github.com/google/uuid"
)

type ProjectRelease struct {
	ent.Schema
}

func (ProjectRelease) Fields() []ent.Field {
	return []ent.Field{
		field.UUID("id", uuid.UUID{}).
			Default(uuid.New).
			Immutable(),

		// The status of the project, which can be one of the following:
		//
		// - review: The project is under review by a moderator
		// - rejected: The project has been rejected by a moderator
		// - approved: The project has been approved by a moderator
		// - archived: The project has been archived by a moderator
		//
		// by default, a project is created with the status of "review"
		// and must be approved by a moderator before it can be published.
		// projects under review cannot be seen by other users beyond
		// members and site admins
		//
		field.Enum("status").
			Values("review", "rejected", "approved", "archived").
			Default("review"),

		field.Enum("type").
			Values("stable", "beta", "alpha").
			Default("stable"),

		// A non unique display name for a project
		//
		field.String("version"),

		// A markdown description of the project
		//
		field.String("description"),

		field.UUID("game_id", uuid.UUID{}),

		field.UUID("project_type_id", uuid.UUID{}),

		// A de-normalized cached count of the number of downloads for this release
		//
		// This is updated by a trigger on the downloads table
		//
		field.Int("download_count").Default(0),

		field.Time("created_at").
			Default(time.Now).
			Immutable(),

		field.Time("updated_at").
			Default(time.Now).
			UpdateDefault(time.Now),
	}
}

func (ProjectRelease) Edges() []ent.Edge {
	return []ent.Edge{
		edge.To("game", Game.Type).
			Field("game_id").
			Unique().
			Required(),

		edge.To("project_type", ProjectType.Type).
			Field("project_type_id").
			Unique().
			Required(),
	}
}
