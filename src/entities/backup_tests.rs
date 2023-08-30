#[cfg(test)]
mod tests{

    use crate::entities::backup::Backup;
    

    #[test]
    fn it_must_build_a_instance_of_the_backup(){

        //arrange
        let content_dowloaded: String = String::from("dummy");
        let content_version: String = String::from("sha:x723-456");

        //action
        let backup = Backup::new(content_dowloaded, content_version);

        //assert
        assert_eq!(backup.content, "dummy");
        assert_eq!(backup.version, "sha:x723-456");
    }

    #[test]
    fn it_must_be_a_valid_state(){
        
        //arrange
        let content_dowloaded: String = String::from("dummy");
        let content_version: String = String::from("sha:x723-456");

        //action
        let backup = Backup::new(content_dowloaded, content_version);

        //assert
        assert_eq!(backup.is_valid(), true);
    }

    #[test]
    fn it_must_be_a_invalid_state(){
        
        //arrange
        let content_dowloaded: String = Default::default();
        let content_version: String = Default::default();

        //action
        let backup = Backup::new(content_dowloaded, content_version);

        //assert
        assert_eq!(backup.is_valid(), false);
    }

}