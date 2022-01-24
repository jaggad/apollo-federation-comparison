package graph

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"

	"github.com/jackedgson/apollo-federation-comparison/tree/master/golang/reviews/graph/generated"
	"github.com/jackedgson/apollo-federation-comparison/tree/master/golang/reviews/graph/model"
)

func (r *userResolver) Reviews(ctx context.Context, obj *model.User) ([]*model.Review, error) {
	var productReviews []*model.Review
	for _, review := range reviews {
		if review.Author.ID == obj.ID {
			productReviews = append(productReviews, review)
		}
	}
	return productReviews, nil
}

// User returns generated.UserResolver implementation.
func (r *Resolver) User() generated.UserResolver { return &userResolver{r} }

type userResolver struct{ *Resolver }
