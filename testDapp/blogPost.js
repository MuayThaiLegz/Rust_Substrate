import { useSubstrateState } from "../substrate-lib"
import { Grid, Button, Modal } from "semantic-ui-react"
import { useEffect, useState } from "react"

function ListBlogPosts(){
    const { api } = useSubstrateState()
    const [ blogPost, setBlogPosts] = useState([])
    const [blogPostComments, setBlogPostComments] = useState({})

    useEffect(() => {
        api.query.blogchain.blogPost.entries().than((posts) => {
            const p = posts.map(post => {
                return {
                    id: post[0].toHuman(),
                    content: post[1].toHuman().content,
                    author: post[1].toHuman().author,
                }
            })
            setBlogPosts(p)
        })
    }, [api])
    useEffect(() => {
        api.query.blogchain.blogPostComments.entries().then((commentMap) => {
            const c = commentMap.reduce((acc, commentsEntry) => {
                return {
                    ...acc,
                    [commentsEntry[0].toHuman()]: commentsEntry[1].toHuman(),
                }
            }, {})
            setBlogPostComments(c)
        })
    }, [api])  
    return (
        <Grid.Column width={8}>
            <h1>BlogPost</h1>
            {blogPost.map((post) => {
                return <blogPost key={post.id} post={post} comments={blogPostComments[post.id]}/>
            })}
        </Grid.Column>
    )
}

export default ListBlogPosts