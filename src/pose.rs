pub trait Pose {
    fn pose(&self, text: &str, key: &str) -> String;
}

pub enum SimplePose {
    StartPose,
    EndPose,
}

impl Pose for SimplePose {
    fn pose(&self, text: &str, key: &str) -> String {
        match self {
            SimplePose::StartPose => {
                format!("{}{}", key, text)
            }
            SimplePose::EndPose => {
                format!("{}{}", text, key)
            }
        }
    }
}

impl Default for SimplePose {
    fn default() -> Self {
        SimplePose::EndPose
    }
}
