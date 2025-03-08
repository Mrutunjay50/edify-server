use futures_util::TryStreamExt;
use mongodb::{bson::Document, error::Result, Collection, Database};
use crate::models::course::Course;

pub struct CourseRepository {
    course_collection: Collection<Course>,
}

impl CourseRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            course_collection: db.collection("courses"),
        }
    }

    // pub async fn create_course(&self, course: Course) -> Result<InsertOneResult> {
    //     self.course_collection.insert_one(course).await
    // }

    pub async fn get_courses(&self, filter: Document) -> Result<Vec<Course>> {
        let mut course_data = self.course_collection.find(filter).await?;
        let mut courses = Vec::new();

        while let Some(course) = course_data.try_next().await? {
            courses.push(course);
        }
        Ok(courses)
    }

    // pub async fn get_one_course(&self, filter: Document) -> Result<Option<Course>> {
    //     let course_data = self.course_collection.find_one(filter).await?;
    //     Ok(course_data)
    // }
}
