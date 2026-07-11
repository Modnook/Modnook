package schema

import (
	"regexp"
	"time"

	"entgo.io/ent"
	"entgo.io/ent/schema/field"

	"github.com/google/uuid"
)

type User struct {
	ent.Schema
}

func (User) Fields() []ent.Field {
	return []ent.Field{
		field.UUID("id", uuid.UUID{}).
			Default(uuid.New).
			Immutable(),

		field.String("username").
			MinLen(3).
			MaxLen(20).
			Match(regexp.MustCompile(`^[a-z0-9_-]+$`)).
			Unique(),

		field.String("displayname").
			MinLen(3).
			MaxLen(50),

		field.String("bio").MaxLen(512).Nillable(),

		// TODO: add a little stricter validation here to ensure
		// that it at least looks something like a url
		//
		field.String("avatar_url").
			MaxLen(255).
			Optional().
			Nillable(),

		field.Enum("role").
			Values("user", "moderator", "admin").
			Default("user"),

		field.Enum("status").
			Values("active", "suspended", "banned", "deleted").
			Default("active"),

		field.String("email").Unique().Optional().Nillable(),

		field.Bool("email_verified").Default(false),

		field.String("phone_number").
			Optional().
			Nillable().
			Unique(),

		field.Bool("phone_number_verified").
			Default(false),

		field.Time("created_at").
			Default(time.Now).
			Immutable(),

		field.Time("updated_at").
			Default(time.Now).
			UpdateDefault(time.Now),
	}
}

func (User) Edges() []ent.Edge {
	return nil
}

func (User) Indexes() []ent.Index {
	return []ent.Index{}
}
