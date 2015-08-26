pub use self::types::{
  CommentId,
  ListIssueCommentsQuery,
  ListRepoCommentsQuery,
  CommentSortables,
  CreateComment,
  EditComment,
  DeleteCommentStatus,
  IssueComment,
  Issue,
};

mod types {
  use rustc_serialize::{
    Decodable,
    Decoder,
    Encodable,
    Encoder,
  };

  use types::{
    Message,
    Url,
    GitTm,
    SortDirection,
    IssueId,
  };

  use users::types::User;

  pub type CommentId = u32;

  #[derive(RustcEncodable, RustcDecodable, Debug)]
  pub struct ListIssueCommentsQuery {
    pub since: GitTm
  }

  #[derive(Debug)]
  pub enum CommentSortables {
    CreatedAt,
    UpdatedAt,
  }

  custom_enum_decode_encode!(
    CommentSortables [
      "created" <=> [CommentSortables::CreatedAt],
      "updated" <=> [CommentSortables::UpdatedAt],
    ]
  );

  #[derive(RustcEncodable, RustcDecodable, Debug)]
  pub struct ListRepoCommentsQuery {
    pub sort: CommentSortables,
    pub direction: SortDirection,
    pub since: GitTm
  }

  #[derive(RustcEncodable, RustcDecodable, Debug)]
  pub struct CreateComment {
    pub body: Message
  }

  pub type EditComment = CreateComment;

  #[allow(dead_code)]
  pub enum DeleteCommentStatus {
    Deleted,
    NotDeleted
  }

  #[derive(RustcDecodable, Debug)]
  pub struct IssueComment {
    pub id: CommentId,
    pub url: Url,
    pub html_url: Url,
    pub body: Message,
    pub user: User,
    pub created_at: GitTm,
    pub updated_at: GitTm
  }

  #[derive(RustcDecodable, Debug)]
  pub struct Issue {
    pub url: Url,
    pub labels_url: Url,
    pub comments_url: Url,
    pub events_url: Url,
    pub html_url: Url,
    pub id: IssueId,
    pub number: u32,
    pub title: Message,
    pub user: User
  }

}
