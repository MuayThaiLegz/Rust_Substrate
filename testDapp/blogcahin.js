import { Grid, Cointainer } from "semantic-ui-react";

function Blogchain() {
    return (
        <Grid.Column width={16}>
            <Cointainer>
                <Grid stackable columns ="equal">
                    <ListBlogPosts />
                    <CreateBlogPost />
                </Grid>
            </Cointainer>
        </Grid.Column>
    )
}

export default Blogchain